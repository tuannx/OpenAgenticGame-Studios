//! Shared-stage coach character — “what to do” overlapped with the live user.
//!
//! Guide and user occupy the same mid-playfield stage (not a bottom strip), each
//! at ~40% opacity so kids can match shapes. Animation runs slower than
//! wall-clock / music BPM so poses stay telegraphable while beds pump energy.

use macroquad::prelude::*;

use crate::juice::{
    draw_character_aura, draw_filled_limb, draw_shared_stage_fx, draw_soft_glow, with_alpha,
    USER_SKELETON_ALPHA,
};

/// Shared mid-playfield stage height — deliberately NOT a bottom-40% band.
pub const GUIDE_HEIGHT_FRACTION: f32 = 0.68;
/// Place leftover slack above/below the stage; 0.5 = true vertical center.
pub const GUIDE_STAGE_SLACK_BIAS: f32 = 0.48;
/// Figure size vs stage height — large enough to read at Zero-Touch distance.
pub const GUIDE_FIGURE_SCALE_FRACTION: f32 = 0.48;
/// Soft stage wash alpha (subtle, not a heavy chrome panel).
pub const GUIDE_FRAME_ALPHA: f32 = 0.18;
/// Guide figure fill alpha — matches user overlay (~40%).
pub const GUIDE_FIGURE_ALPHA: f32 = USER_SKELETON_ALPHA;
/// Soft outer halo alpha around the coach.
pub const GUIDE_HALO_ALPHA: f32 = 0.28;
/// Wall-clock multiplier for coach joint motion (music can be faster).
pub const GUIDE_ANIM_RATE: f32 = 0.42;
/// How long each dance-verb demo holds before cycling (seconds of wall time).
pub const GUIDE_DANCE_VERB_SECONDS: f32 = 5.2;

const TAU: f32 = std::f32::consts::TAU;

/// Guided action the coach demonstrates — kept in sync with VO cue ids.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum GuidePose {
    Idle,
    Jump,
    Clap,
    Freeze,
    Squat,
    RaiseHands,
    DodgeLeft,
    DodgeRight,
    Go,
    Dance,
}

impl GuidePose {
    /// Map a Kokoro / party-voice cue id string to a coach pose.
    pub fn from_voice_cue(id: &str) -> Self {
        match id {
            "jump" | "ritual_jump" => Self::Jump,
            "clap_replay" => Self::Clap,
            "freeze" => Self::Freeze,
            "drop" => Self::Squat,
            "lava" | "ritual_lava" => Self::Jump,
            "raise_hand" | "hold" => Self::RaiseHands,
            "duo_invite" => Self::RaiseHands,
            "dance" | "lets_go" => Self::Dance,
            "go" | "count_3" | "count_2" | "count_1" => Self::Go,
            "perfect" | "celebrate" => Self::Jump,
            "nice_try" => Self::Idle,
            _ => Self::Idle,
        }
    }

    /// Supernova dance verb index (matches `GestureCue::from_index` cadence).
    pub fn from_dance_verb(index: usize) -> Self {
        match index % 8 {
            0 => Self::Jump,
            1 => Self::Clap,
            2 | 4 | 5 | 7 => Self::RaiseHands,
            3 => Self::Go,
            6 => Self::Dance,
            _ => Self::Dance,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GuideCoachLayout {
    pub frame: Rect,
    pub figure_center: Vec2,
    pub figure_scale: f32,
}

/// Mid-playfield shared stage where guide and user overlap (full width).
///
/// Rejects the legacy bottom-40% strip (`y ≈ 0.6h`, `h ≈ 0.4h`). Stage covers
/// ~68% of viewport height and is vertically centered with a slight grounding
/// bias — figures sit mid-screen, not in a bottom band only.
pub fn guide_coach_layout(screen_w: f32, screen_h: f32) -> GuideCoachLayout {
    let band_h = screen_h * GUIDE_HEIGHT_FRACTION;
    let slack = (screen_h - band_h).max(0.0);
    let frame_y = slack * GUIDE_STAGE_SLACK_BIAS;
    let frame = Rect::new(0.0, frame_y, screen_w, band_h);
    let figure_scale = band_h * GUIDE_FIGURE_SCALE_FRACTION;
    GuideCoachLayout {
        frame,
        figure_center: vec2(frame.x + frame.w * 0.5, frame.y + frame.h * 0.50),
        figure_scale,
    }
}

/// Same region as the coach — pink “you” stacks here at matching opacity.
pub fn user_overlay_bounds(screen_w: f32, screen_h: f32) -> Rect {
    guide_coach_layout(screen_w, screen_h).frame
}

pub fn draw_guide_coach(pose: GuidePose, time: f32, audio_pulse: f32, reduce_motion: bool) {
    draw_guide_coach_in(
        Rect::new(0.0, 0.0, screen_width(), screen_height()),
        pose,
        time,
        audio_pulse,
        reduce_motion,
    );
}

pub fn draw_guide_coach_in(
    viewport: Rect,
    pose: GuidePose,
    time: f32,
    audio_pulse: f32,
    reduce_motion: bool,
) {
    let layout = guide_coach_layout(viewport.w, viewport.h);
    let frame = Rect::new(
        viewport.x + layout.frame.x,
        viewport.y + layout.frame.y,
        layout.frame.w,
        layout.frame.h,
    );
    let figure_center = vec2(
        viewport.x + layout.figure_center.x,
        viewport.y + layout.figure_center.y,
    );
    draw_guide_stage_wash(frame, pose);
    let bounce = if reduce_motion {
        1.0
    } else {
        1.0 + audio_pulse * 0.04 + (time * GUIDE_ANIM_RATE * 3.2).sin().abs() * 0.02
    };
    let scale = layout.figure_scale * bounce;
    draw_shared_stage_fx(
        figure_center,
        scale * 1.35,
        time,
        audio_pulse,
        pose_accent(pose),
        reduce_motion,
    );
    draw_guide_figure(
        figure_center,
        scale,
        pose,
        time,
        reduce_motion,
    );
}

fn draw_guide_stage_wash(frame: Rect, pose: GuidePose) {
    let accent = pose_accent(pose);
    draw_rectangle(
        frame.x,
        frame.y,
        frame.w,
        frame.h,
        Color::new(0.02, 0.04, 0.12, GUIDE_FRAME_ALPHA),
    );
    draw_ellipse(
        frame.x + frame.w * 0.5,
        frame.y + frame.h * 0.82,
        frame.w * 0.28,
        frame.h * 0.08,
        0.0,
        with_alpha(accent, 0.16),
    );
}

fn pose_accent(pose: GuidePose) -> Color {
    match pose {
        GuidePose::Jump | GuidePose::Go => Color::from_rgba(103, 232, 249, 255),
        GuidePose::Clap => Color::from_rgba(255, 202, 58, 255),
        GuidePose::Freeze => Color::from_rgba(140, 220, 255, 255),
        GuidePose::Squat => Color::from_rgba(255, 140, 66, 255),
        GuidePose::RaiseHands | GuidePose::Dance => Color::from_rgba(196, 181, 253, 255),
        GuidePose::DodgeLeft | GuidePose::DodgeRight => Color::from_rgba(251, 113, 133, 255),
        GuidePose::Idle => Color::from_rgba(148, 163, 184, 255),
    }
}

fn draw_guide_figure(
    center: Vec2,
    scale: f32,
    pose: GuidePose,
    time: f32,
    reduce_motion: bool,
) {
    let accent = pose_accent(pose);
    let fill = with_alpha(accent, GUIDE_FIGURE_ALPHA);
    let soft = with_alpha(accent, GUIDE_HALO_ALPHA);
    draw_character_aura(center, scale * 0.95, accent, 0.25);
    draw_soft_glow(center, scale * 1.05, soft, 3);

    let anim = if reduce_motion {
        0.0
    } else {
        time * GUIDE_ANIM_RATE
    };
    let (
        head,
        l_shoulder,
        r_shoulder,
        l_elbow,
        r_elbow,
        l_hand,
        r_hand,
        l_hip,
        r_hip,
        l_knee,
        r_knee,
        l_foot,
        r_foot,
    ) = figure_joints(center, scale, pose, anim);

    let torso = with_alpha(accent, GUIDE_FIGURE_ALPHA);
    let limb_t = scale * 0.14;
    draw_filled_limb(l_shoulder, r_shoulder, limb_t * 0.85, torso);
    draw_filled_limb(
        (l_shoulder + r_shoulder) * 0.5,
        (l_hip + r_hip) * 0.5,
        limb_t * 1.15,
        torso,
    );
    draw_filled_limb(l_hip, r_hip, limb_t * 0.8, torso);

    draw_filled_limb(l_shoulder, l_elbow, limb_t, fill);
    draw_filled_limb(l_elbow, l_hand, limb_t * 0.92, fill);
    draw_filled_limb(r_shoulder, r_elbow, limb_t, fill);
    draw_filled_limb(r_elbow, r_hand, limb_t * 0.92, fill);
    draw_filled_limb(l_hip, l_knee, limb_t * 1.05, fill);
    draw_filled_limb(l_knee, l_foot, limb_t, fill);
    draw_filled_limb(r_hip, r_knee, limb_t * 1.05, fill);
    draw_filled_limb(r_knee, r_foot, limb_t, fill);

    draw_circle(head.x, head.y, scale * 0.18, with_alpha(WHITE, GUIDE_FIGURE_ALPHA + 0.08));
    draw_circle(head.x, head.y, scale * 0.18, fill);
    draw_circle(head.x, head.y, scale * 0.07, with_alpha(WHITE, 0.55));
    draw_circle(l_hand.x, l_hand.y, scale * 0.09, with_alpha(WHITE, GUIDE_FIGURE_ALPHA + 0.15));
    draw_circle(r_hand.x, r_hand.y, scale * 0.09, with_alpha(WHITE, GUIDE_FIGURE_ALPHA + 0.15));

    // Pose-specific sparkle — still no text.
    match pose {
        GuidePose::Jump => {
            let lift = ((anim * 3.2).sin().abs()) * scale * 0.12;
            draw_triangle(
                vec2(center.x, head.y - scale * 0.42 - lift),
                vec2(center.x - scale * 0.12, head.y - scale * 0.22 - lift),
                vec2(center.x + scale * 0.12, head.y - scale * 0.22 - lift),
                with_alpha(WHITE, 0.55),
            );
        }
        GuidePose::Freeze => {
            draw_rectangle(
                (l_shoulder.x + r_shoulder.x) * 0.5 - scale * 0.38,
                l_shoulder.y - scale * 0.06,
                scale * 0.76,
                scale * 0.1,
                with_alpha(WHITE, 0.5),
            );
        }
        GuidePose::Squat => {
            draw_triangle(
                vec2(center.x, (l_hip.y + r_hip.y) * 0.5 + scale * 0.55),
                vec2(center.x - scale * 0.2, (l_hip.y + r_hip.y) * 0.5 + scale * 0.28),
                vec2(center.x + scale * 0.2, (l_hip.y + r_hip.y) * 0.5 + scale * 0.28),
                with_alpha(Color::from_rgba(255, 202, 58, 255), 0.55),
            );
        }
        GuidePose::Clap => {
            let gap = (anim * 5.5).sin().abs() * scale * 0.06;
            draw_circle(
                center.x - scale * 0.16 - gap,
                center.y - scale * 0.05,
                scale * 0.12,
                fill,
            );
            draw_circle(
                center.x + scale * 0.16 + gap,
                center.y - scale * 0.05,
                scale * 0.12,
                fill,
            );
        }
        _ => {}
    }
}

type Joints = (
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
    Vec2,
);

fn figure_joints(center: Vec2, scale: f32, pose: GuidePose, time: f32) -> Joints {
    // Slow bob — telegraphable, not frantic.
    let bob = (time * 2.4).sin() * scale * 0.025;
    let mut head = vec2(center.x, center.y - scale * 0.55 + bob);
    let mut l_shoulder = vec2(center.x - scale * 0.16, center.y - scale * 0.22 + bob);
    let mut r_shoulder = vec2(center.x + scale * 0.16, center.y - scale * 0.22 + bob);
    let mut l_hip = vec2(center.x - scale * 0.12, center.y + scale * 0.16 + bob);
    let mut r_hip = vec2(center.x + scale * 0.12, center.y + scale * 0.16 + bob);
    let mut l_hand = vec2(center.x - scale * 0.38, center.y + scale * 0.05);
    let mut r_hand = vec2(center.x + scale * 0.38, center.y + scale * 0.05);
    let mut l_foot = vec2(center.x - scale * 0.28, center.y + scale * 0.72);
    let mut r_foot = vec2(center.x + scale * 0.28, center.y + scale * 0.72);

    match pose {
        GuidePose::Idle => {}
        GuidePose::Jump => {
            // Long hold at apex so kids can copy the shape.
            let lift = ((time * 3.0).sin().abs()).powf(0.65) * scale * 0.30;
            head.y -= lift;
            l_shoulder.y -= lift;
            r_shoulder.y -= lift;
            l_hip.y -= lift;
            r_hip.y -= lift;
            l_hand = vec2(center.x - scale * 0.42, l_shoulder.y - scale * 0.22);
            r_hand = vec2(center.x + scale * 0.42, r_shoulder.y - scale * 0.22);
            l_foot = vec2(center.x - scale * 0.22, l_hip.y + scale * 0.48);
            r_foot = vec2(center.x + scale * 0.22, r_hip.y + scale * 0.48);
        }
        GuidePose::Clap => {
            let close = (time * 5.5).sin().abs();
            l_hand = vec2(
                center.x - scale * (0.22 - close * 0.12),
                center.y - scale * 0.05,
            );
            r_hand = vec2(
                center.x + scale * (0.22 - close * 0.12),
                center.y - scale * 0.05,
            );
        }
        GuidePose::Freeze => {
            l_hand = vec2(center.x - scale * 0.45, l_shoulder.y);
            r_hand = vec2(center.x + scale * 0.45, r_shoulder.y);
            l_foot = vec2(center.x - scale * 0.2, center.y + scale * 0.7);
            r_foot = vec2(center.x + scale * 0.2, center.y + scale * 0.7);
        }
        GuidePose::Squat => {
            head.y += scale * 0.12;
            l_shoulder.y += scale * 0.18;
            r_shoulder.y += scale * 0.18;
            l_hip.y += scale * 0.08;
            r_hip.y += scale * 0.08;
            l_hand = vec2(center.x - scale * 0.35, l_hip.y - scale * 0.05);
            r_hand = vec2(center.x + scale * 0.35, r_hip.y - scale * 0.05);
            l_foot = vec2(center.x - scale * 0.32, center.y + scale * 0.62);
            r_foot = vec2(center.x + scale * 0.32, center.y + scale * 0.62);
        }
        GuidePose::RaiseHands => {
            let wave = (time * 2.8).sin() * scale * 0.06;
            l_hand = vec2(center.x - scale * 0.4 + wave, head.y - scale * 0.35);
            r_hand = vec2(center.x + scale * 0.4 - wave, head.y - scale * 0.35);
        }
        GuidePose::DodgeLeft => {
            let lean = scale * 0.22;
            head.x -= lean;
            l_shoulder.x -= lean;
            r_shoulder.x -= lean;
            l_hip.x -= lean * 0.6;
            r_hip.x -= lean * 0.6;
            l_hand = vec2(l_shoulder.x - scale * 0.4, l_shoulder.y + scale * 0.05);
            r_hand = vec2(r_shoulder.x + scale * 0.25, r_shoulder.y + scale * 0.2);
            l_foot = vec2(l_hip.x - scale * 0.35, center.y + scale * 0.7);
            r_foot = vec2(r_hip.x + scale * 0.15, center.y + scale * 0.7);
        }
        GuidePose::DodgeRight => {
            let lean = scale * 0.22;
            head.x += lean;
            l_shoulder.x += lean;
            r_shoulder.x += lean;
            l_hip.x += lean * 0.6;
            r_hip.x += lean * 0.6;
            l_hand = vec2(l_shoulder.x - scale * 0.25, l_shoulder.y + scale * 0.2);
            r_hand = vec2(r_shoulder.x + scale * 0.4, r_shoulder.y + scale * 0.05);
            l_foot = vec2(l_hip.x - scale * 0.15, center.y + scale * 0.7);
            r_foot = vec2(r_hip.x + scale * 0.35, center.y + scale * 0.7);
        }
        GuidePose::Go => {
            let march = (time * 3.6).sin();
            l_hand = vec2(center.x - scale * 0.3, l_shoulder.y + march * scale * 0.12);
            r_hand = vec2(center.x + scale * 0.3, r_shoulder.y - march * scale * 0.12);
            l_foot = vec2(
                center.x - scale * 0.22,
                center.y + scale * (0.62 - march * 0.10),
            );
            r_foot = vec2(
                center.x + scale * 0.22,
                center.y + scale * (0.62 + march * 0.10),
            );
        }
        GuidePose::Dance => {
            let wiggle = (time * 2.6).sin();
            head.x += wiggle * scale * 0.03;
            l_hand = vec2(
                center.x - scale * 0.42,
                l_shoulder.y - scale * 0.15 + wiggle * scale * 0.10,
            );
            r_hand = vec2(
                center.x + scale * 0.42,
                r_shoulder.y - scale * 0.15 - wiggle * scale * 0.10,
            );
            l_foot = vec2(
                center.x - scale * 0.25 + wiggle * scale * 0.05,
                center.y + scale * 0.7,
            );
            r_foot = vec2(
                center.x + scale * 0.25 - wiggle * scale * 0.05,
                center.y + scale * 0.7,
            );
        }
    }

    let l_elbow = (l_shoulder + l_hand) * 0.5 + vec2(0.0, scale * 0.04);
    let r_elbow = (r_shoulder + r_hand) * 0.5 + vec2(0.0, scale * 0.04);
    let l_knee = (l_hip + l_foot) * 0.5 + vec2(-scale * 0.02, 0.0);
    let r_knee = (r_hip + r_foot) * 0.5 + vec2(scale * 0.02, 0.0);

    let _ = TAU;
    (
        head, l_shoulder, r_shoulder, l_elbow, r_elbow, l_hand, r_hand, l_hip, r_hip, l_knee,
        r_knee, l_foot, r_foot,
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_stage_overlaps_guide_and_user_bounds() {
        for (w, h) in [(390.0, 844.0), (667.0, 375.0), (1440.0, 784.0)] {
            let layout = guide_coach_layout(w, h);
            let overlay = user_overlay_bounds(w, h);
            assert!((layout.frame.x - overlay.x).abs() < 0.5);
            assert!((layout.frame.y - overlay.y).abs() < 0.5);
            assert!((layout.frame.w - overlay.w).abs() < 0.5);
            assert!((layout.frame.h - overlay.h).abs() < 0.5);
            assert!((layout.frame.h - h * GUIDE_HEIGHT_FRACTION).abs() < 0.5);
            assert!(layout.figure_scale > h * 0.20);
            assert!((GUIDE_FIGURE_ALPHA - 0.40).abs() < 0.001);
        }
    }

    #[test]
    fn shared_stage_is_mid_playfield_not_bottom_strip() {
        for (w, h) in [(390.0, 844.0), (667.0, 375.0), (1440.0, 784.0)] {
            let layout = guide_coach_layout(w, h);
            // Legacy bottom-40% was y≈0.6h with h≈0.4h — reject that geometry.
            assert!(
                layout.frame.y < h * 0.35,
                "stage top must sit in upper half (got y/h={})",
                layout.frame.y / h
            );
            assert!(
                layout.frame.h >= h * 0.50 && layout.frame.h <= h * 0.75,
                "stage height must be mid-playfield 50–75% (got h/H={})",
                layout.frame.h / h
            );
            assert!(
                layout.figure_center.y > h * 0.35 && layout.figure_center.y < h * 0.65,
                "figure center must be mid-screen (got cy/h={})",
                layout.figure_center.y / h
            );
            // Must extend through the vertical middle, not sit only below 60%.
            assert!(layout.frame.y + layout.frame.h > h * 0.70);
            assert!(layout.frame.y < h * 0.45);
            let _ = w;
        }
    }

    #[test]
    fn guide_anim_is_slower_than_music_clock() {
        assert!(GUIDE_ANIM_RATE < 0.55);
        assert!(GUIDE_DANCE_VERB_SECONDS >= 5.0);
    }

    #[test]
    fn voice_cues_map_to_expected_poses() {
        assert_eq!(GuidePose::from_voice_cue("jump"), GuidePose::Jump);
        assert_eq!(GuidePose::from_voice_cue("freeze"), GuidePose::Freeze);
        assert_eq!(GuidePose::from_voice_cue("drop"), GuidePose::Squat);
        assert_eq!(GuidePose::from_voice_cue("raise_hand"), GuidePose::RaiseHands);
        assert_eq!(GuidePose::from_voice_cue("clap_replay"), GuidePose::Clap);
        assert_eq!(GuidePose::from_voice_cue("lava"), GuidePose::Jump);
        assert_eq!(GuidePose::from_voice_cue("dance"), GuidePose::Dance);
        assert_eq!(GuidePose::from_voice_cue("go"), GuidePose::Go);
    }

    #[test]
    fn dance_verb_cycle_covers_core_actions() {
        let poses: Vec<_> = (0..8).map(GuidePose::from_dance_verb).collect();
        assert!(poses.contains(&GuidePose::Jump));
        assert!(poses.contains(&GuidePose::Clap));
        assert!(poses.contains(&GuidePose::RaiseHands));
    }
}
