use super::{
    add_custom_sprint_section, add_template, add_template_sprint, add_template_sprint_sections,
};
use super::super::super::types::AppError;
use rusqlite::Connection;

pub fn seed(
    conn: &Connection,
    section_map: &std::collections::HashMap<String, i64>,
    sprint_map: &std::collections::HashMap<String, i64>,
) -> Result<(), AppError> {
    let planning = section_map.get("planning").copied().unwrap_or(0);
    let security = section_map.get("security").copied().unwrap_or(0);
    let testing = section_map.get("testing").copied().unwrap_or(0);
    let cicd = section_map.get("cicd").copied().unwrap_or(0);
    let docs = section_map.get("docs").copied().unwrap_or(0);
    let quality = section_map.get("quality").copied().unwrap_or(0);
    let performance = section_map.get("performance").copied().unwrap_or(0);
    let monitoring = section_map.get("monitoring").copied().unwrap_or(0);

    let planning_setup = sprint_map.get("planning_setup").copied().unwrap_or(0);
    let security_quality = sprint_map.get("security_quality").copied().unwrap_or(0);
    let testing_qa = sprint_map.get("testing_qa").copied().unwrap_or(0);
    let cicd_deploy = sprint_map.get("cicd_deploy").copied().unwrap_or(0);
    let monitoring_ops = sprint_map.get("monitoring_ops").copied().unwrap_or(0);
    let perf_sprint = sprint_map.get("performance").copied().unwrap_or(0);

    // === Game Development ===
    let tpl = add_template(
        conn,
        "Game Development",
        "Unity / Unreal / Godot / Custom engine — PC, console, mobile, and VR/AR game checklist",
        "#ef4444",
    )?;

    let s1 = add_template_sprint(
        conn,
        tpl,
        "Planning & Setup",
        "Game design, engine setup, and project structure",
        0,
    )?;
    add_template_sprint_sections(conn, s1, &[planning])?;
    add_custom_sprint_section(
        conn,
        s1,
        "Game Design Document",
        "Core game mechanics and design decisions",
        "#ef4444",
        &[
            (
                "Game design document written",
                "Core loop, mechanics, progression, and win/lose conditions",
            ),
            (
                "Target platform(s) defined",
                "PC, console, mobile, VR/AR with technical requirements",
            ),
            (
                "Art style and direction established",
                "Visual reference, mood board, and style guide",
            ),
            (
                "Audio direction defined",
                "Music style, SFX approach, and voice-over requirements",
            ),
            (
                "Technical requirements documented",
                "Target FPS, resolution, memory budget, and storage size",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s1,
        "Engine Setup",
        "Game engine configuration and project structure",
        "#f97316",
        &[
            (
                "Game engine project initialized",
                "Unity, Unreal, Godot, or custom engine with settings",
            ),
            (
                "Version control configured for game assets",
                "Git LFS, Perforce, or Plastic SCM for binary files",
            ),
            (
                "Project folder structure organized",
                "Scenes, scripts, assets, prefabs, and config folders",
            ),
            (
                "Build pipeline configured",
                "Platform-specific build targets and output directories",
            ),
            (
                "Development tools integrated",
                "Profiler, debugger, and hot-reload for scripts",
            ),
        ],
    )?;

    let s2 = add_template_sprint(
        conn,
        tpl,
        "Core Game Loop",
        "Game state, input, and main gameplay loop",
        1,
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Game Loop & State",
        "Core game architecture and state management",
        "#ef4444",
        &[
            (
                "Game state machine implemented",
                "Menu, playing, paused, game over states with transitions",
            ),
            (
                "Input system configured",
                "Keyboard, mouse, gamepad, and touch input handling",
            ),
            (
                "Game loop timing established",
                "Fixed timestep for physics, variable for rendering",
            ),
            (
                "Save/load system designed",
                "Serialization format, save slots, and auto-save triggers",
            ),
            (
                "Scene/level loading implemented",
                "Async loading with progress indicators",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s2,
        "Player Controller",
        "Character movement, camera, and interaction",
        "#f59e0b",
        &[
            (
                "Player movement implemented",
                "Walking, running, jumping with proper physics",
            ),
            (
                "Camera system configured",
                "Follow, orbit, or fixed camera with smooth transitions",
            ),
            (
                "Player interaction system built",
                "Object interaction, inventory, and dialogue triggers",
            ),
            (
                "Animation controller set up",
                "State machine for idle, walk, run, jump, and attack animations",
            ),
            (
                "Player health/damage system implemented",
                "HP, damage calculation, and death/respawn logic",
            ),
        ],
    )?;

    let s3 = add_template_sprint(
        conn,
        tpl,
        "Physics & Collision",
        "Physics engine, collision detection, and responses",
        2,
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Physics System",
        "Physics engine configuration and tuning",
        "#14b8a6",
        &[
            (
                "Physics engine configured",
                "Gravity, timestep, and solver iterations tuned",
            ),
            (
                "Collision layers and masks defined",
                "Layer-based collision filtering for performance",
            ),
            (
                "Rigidbody components configured",
                "Mass, drag, and constraints for dynamic objects",
            ),
            (
                "Trigger volumes implemented",
                "Zone detection for events, cutscenes, and loading",
            ),
            (
                "Physics materials assigned",
                "Friction, bounciness, and surface effects",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s3,
        "Game Mechanics",
        "Core gameplay systems and rules",
        "#06b6d4",
        &[
            (
                "Combat or core mechanic implemented",
                "Attack, defend, or primary gameplay action",
            ),
            (
                "AI behavior trees or state machines",
                "Enemy AI with patrol, chase, attack, and flee states",
            ),
            (
                "Scoring and progression system",
                "Points, levels, experience, and unlock system",
            ),
            (
                "Inventory or resource management",
                "Item collection, storage, and usage system",
            ),
            (
                "Environmental interactions",
                "Destructible objects, puzzles, or physics-based gameplay",
            ),
        ],
    )?;

    let s4 = add_template_sprint(
        conn,
        tpl,
        "Audio System",
        "Music, sound effects, and audio mixing",
        3,
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "Audio Implementation",
        "Sound system and audio management",
        "#8b5cf6",
        &[
            (
                "Audio manager implemented",
                "Centralized audio playback with pooling",
            ),
            (
                "Background music system configured",
                "Looping, crossfade, and dynamic music layers",
            ),
            (
                "Sound effects integrated",
                "Footsteps, impacts, UI sounds, and environmental audio",
            ),
            (
                "Spatial audio configured",
                "3D sound positioning and distance attenuation",
            ),
            (
                "Audio mixer with channels",
                "Master, music, SFX, and voice channels with volume control",
            ),
        ],
    )?;
    add_custom_sprint_section(
        conn,
        s4,
        "UI & HUD",
        "Game user interface and heads-up display",
        "#a855f7",
        &[
            (
                "Main menu and pause menu built",
                "Start, settings, credits, and quit functionality",
            ),
            (
                "HUD elements implemented",
                "Health, ammo, score, minimap, and objective markers",
            ),
            (
                "Settings menu with audio controls",
                "Volume sliders, music/SFX toggles, and quality settings",
            ),
            (
                "Loading screen with tips",
                "Progress bar, loading tips, and background art",
            ),
            (
                "Game over and victory screens",
                "Results display, replay option, and share functionality",
            ),
        ],
    )?;

    let s5 = add_template_sprint(
        conn,
        tpl,
        "Asset Pipeline",
        "Art, animation, and content workflow",
        4,
    )?;
    add_custom_sprint_section(
        conn,
        s5,
        "Asset Management",
        "Import, optimization, and organization",
        "#ec4899",
        &[
            (
                "Asset import pipeline configured",
                "FBX, PNG, WAV import settings and compression",
            ),
            (
                "Texture optimization applied",
                "Mipmaps, compression formats, and atlasing",
            ),
            (
                "Model LOD system implemented",
                "Level-of-detail meshes for distance-based rendering",
            ),
            (
                "Animation pipeline established",
                "Import, retargeting, and blending workflows",
            ),
            (
                "Asset bundle or addressable system",
                "Dynamic loading and memory management for assets",
            ),
        ],
    )?;

    let s6 = add_template_sprint(
        conn,
        tpl,
        "Security & Anti-Cheat",
        "Game security and cheat prevention",
        5,
    )?;
    add_template_sprint_sections(conn, s6, &[security])?;
    add_custom_sprint_section(
        conn,
        s6,
        "Game Security",
        "Anti-cheat and save file protection",
        "#ef4444",
        &[
            (
                "Save file integrity checks",
                "Checksums or encryption to prevent save editing",
            ),
            (
                "Server-side validation for critical actions",
                "Damage, scoring, and progression verified server-side",
            ),
            (
                "Memory scan protection",
                "Basic anti-tamper for single-player games",
            ),
            (
                "Input validation on networked actions",
                "Server-authoritative movement and combat",
            ),
            (
                "DRM or licensing check implemented",
                "Platform-specific DRM (Steam, Epic, etc.)",
            ),
        ],
    )?;

    let s7 = add_template_sprint(
        conn,
        tpl,
        "Testing & QA",
        "Game testing, playtesting, and bug fixing",
        6,
    )?;
    add_template_sprint_sections(conn, s7, &[testing])?;
    add_custom_sprint_section(
        conn,
        s7,
        "Game Testing",
        "Gameplay and technical testing",
        "#10b981",
        &[
            (
                "Automated unit tests for game logic",
                "Core mechanics, scoring, and state transitions",
            ),
            (
                "Playtesting sessions conducted",
                "External testers with feedback collection",
            ),
            (
                "Performance profiling on target hardware",
                "FPS, memory, and load times on minimum specs",
            ),
            (
                "Compatibility testing across platforms",
                "Different GPUs, OS versions, and input devices",
            ),
            (
                "Edge case and exploit testing",
                "Sequence breaking, soft locks, and infinite loops",
            ),
        ],
    )?;

    let s8 = add_template_sprint(
        conn,
        tpl,
        "Performance Optimization",
        "Frame rate, memory, and loading optimization",
        7,
    )?;
    add_template_sprint_sections(conn, s8, &[performance])?;
    add_custom_sprint_section(
        conn,
        s8,
        "Game Performance",
        "Rendering and runtime optimization",
        "#f97316",
        &[
            (
                "Draw call batching implemented",
                "Static and dynamic batching for reduced draw calls",
            ),
            (
                "Occlusion culling configured",
                "Frustum and occlusion culling for hidden objects",
            ),
            (
                "Garbage collection optimized",
                "Object pooling and allocation-free hot paths",
            ),
            (
                "Texture streaming implemented",
                "Async texture loading with mip streaming",
            ),
            (
                "Target FPS achieved on minimum specs",
                "Consistent frame rate at target resolution",
            ),
        ],
    )?;

    let s9 = add_template_sprint(
        conn,
        tpl,
        "CI/CD & Deployment",
        "Build automation and platform submission",
        8,
    )?;
    add_template_sprint_sections(conn, s9, &[cicd, docs])?;

    let s10 = add_template_sprint(
        conn,
        tpl,
        "Launch & Post-Launch",
        "Store submission and live operations",
        9,
    )?;
    add_template_sprint_sections(conn, s10, &[monitoring, docs])?;
    add_custom_sprint_section(
        conn,
        s10,
        "Platform Submission",
        "Store requirements and certification",
        "#ef4444",
        &[
            (
                "Platform certification requirements met",
                "TRC/XR compliance for console platforms",
            ),
            (
                "Store page assets prepared",
                "Screenshots, trailer, description, and tags",
            ),
            (
                "Achievement/trophy system integrated",
                "Platform-specific achievement APIs",
            ),
            (
                "Leaderboard or online features configured",
                "Platform leaderboards or custom backend",
            ),
            (
                "Day-one patch pipeline ready",
                "Post-launch update mechanism tested",
            ),
        ],
    )?;

    Ok(())
}
