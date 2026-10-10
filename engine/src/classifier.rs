use std::collections::BTreeMap;

/// Classify an AnimCJK median stroke into one of the 5 canonical types.
///
/// AnimCJK uses a flipped coordinate system where y=0 is at the *bottom* of the
/// character and y increases *upward*. This is opposite to the usual SVG
/// convention. To use conventional writing geometry we flip the y axis.
///
/// Conventional writing geometry (y up):
///   橫 (heng)    - horizontal, left to right,        angle ≈ 0°
///   豎 (shu)     - vertical, top to bottom,         angle ≈ -90°
///   撇 (pie)     - left-falling, top-right to bottom-left, angle ≈ +135°
///   點/捺 (na)   - right-falling, top-left to bottom-right, angle ≈ +45°
///   折 (zhe)     - changes direction: hook or turn
///
/// This is a simplified port of the geometric logic used by HanziLookupJS.
pub fn classify_stroke(median: &[(i32, i32)]) -> Option<u8> {
    if median.len() < 2 {
        return None;
    }

    let (x0, y0) = median[0];
    let (xn, yn) = median[median.len() - 1];
    let dx = (xn - x0) as f64;
    // Flip y so conventional geometry works.
    let dy = -((yn - y0) as f64);

    if has_turn(median, dx, dy) {
        return Some(5); // 折
    }

    let angle = dy.atan2(dx).to_degrees();

    // 橫: left to right, near horizontal (dominant horizontal movement)
    if (-25.0..=25.0).contains(&angle) && dx.abs() >= dy.abs() * 2.0 {
        return Some(7);
    }

    // 豎: top to bottom, near vertical (dominant vertical movement)
    if (-100.0..=-80.0).contains(&angle) || (80.0..=100.0).contains(&angle) {
        if dy.abs() >= dx.abs() * 2.0 {
            return Some(8);
        }
    }

    // 撇: top-right to bottom-left.
    // Sum vector has dx < 0 and dy > 0 (flipped), so angle in (80, 170].
    if angle > 80.0 && angle <= 170.0 && dx < 0.0 {
        return Some(9);
    }

    // 點/捺: top-left to bottom-right.
    // Sum vector has dx > 0 and dy > 0 (flipped), so angle in (10, 80).
    if angle > 10.0 && angle < 80.0 && dx > 0.0 {
        return Some(4);
    }

    // Fallback by dominant direction.
    if dx.abs() >= dy.abs() * 2.0 {
        return Some(7);
    }
    if dy.abs() >= dx.abs() * 2.0 {
        return Some(8);
    }

    // Ambiguous short strokes
    if dx > 0.0 {
        return Some(4);
    }
    if dx < 0.0 {
        return Some(9);
    }
    Some(8)
}

/// Detect a turn/hook: any segment reverses direction by more than 90°
/// relative to the previous segment, or the polyline bends back strongly.
fn has_turn(median: &[(i32, i32)], dx_total: f64, dy_total: f64) -> bool {
    if median.len() < 3 {
        return false;
    }
    let total_len = (dx_total * dx_total + dy_total * dy_total).sqrt();
    if total_len < 1.0 {
        return false;
    }

    let mut prev_dx = 0.0f64;
    let mut prev_dy = 0.0f64;
    let mut prev_len = 0.0f64;

    for i in 1..median.len() {
        let (x1, y1) = median[i - 1];
        let (x2, y2) = median[i];
        let dx = (x2 - x1) as f64;
        // Flip y for consistency.
        let dy = -((y2 - y1) as f64);
        let seg_len = (dx * dx + dy * dy).sqrt();
        if seg_len < 1.0 {
            continue;
        }

        if prev_len > 0.0 {
            let dot = (dx * prev_dx + dy * prev_dy) / (seg_len * prev_len);
            if dot < -0.2 {
                return true;
            }
            let cross = dx * prev_dy - dy * prev_dx;
            let bend = cross.abs() / (seg_len * prev_len);
            if total_len < 30.0 && bend > 0.5 {
                return true;
            }
        }
        prev_dx = dx;
        prev_dy = dy;
        prev_len = seg_len;
    }

    false
}

/// Compute total length of the polyline (in flipped y convention).
pub fn total_length(median: &[(i32, i32)]) -> f64 {
    let mut len = 0.0;
    for i in 1..median.len() {
        let (x1, y1) = median[i - 1];
        let (x2, y2) = median[i];
        let dx = (x2 - x1) as f64;
        let dy = -((y2 - y1) as f64);
        len += (dx * dx + dy * dy).sqrt();
    }
    len
}

/// Map from common CJK strokes (used in raw data) to our canonical types.
/// AnimCJK doesn't label strokes this way; we rely on geometry instead.
/// This map is kept for future overrides only.
pub fn stroke_overrides() -> BTreeMap<String, u8> {
    BTreeMap::new()
}
