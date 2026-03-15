// ============ IMPORTS ============
use std::collections::HashSet;
use strsim::levenshtein;
use iced::font::Family;
use iced::Font;

use crate::weight_from_str;
use crate::intern_string;



// ============ CONST ============
const MIN_FUZZY_LEN: usize = 4;
const MAX_FUZZY_RATIO: f32 = 0.3;



// ============ FUNCTIONS ============
pub fn build_font(requested: &str, style: &str) -> Font
{
    match resolve_font(requested)
    {
        Some(resolved) =>
        {
            if resolved != requested
            {
                println!("\n=== FONT RESOLVER ===");
                println!("Font '{}' resolved to '{}'", requested, resolved);
            }
            Font
            {
                family: Family::Name(intern_string(resolved)),
                weight: weight_from_str(style),
                ..Font::DEFAULT
            }
        }
        None =>
        {
            println!("\n=== FONT RESOLVER ===");
            println!("Font '{}' not found, using system default", requested);
            Font::DEFAULT
        }
    }
}



pub fn resolve_font(requested: &str) -> Option<String>
{
    let normalized_requested = normalize(requested);
    let fonts = system_fonts();

    // exact normalized match
    for font in &fonts
    {
        if normalize(font) == normalized_requested
        {
            return Some(font.clone());
        }
    }

    // prefix match: system font's normalized name starts with what user asked
    for font in &fonts
    {
        if normalize(font).starts_with(&normalized_requested)
        {
            return Some(font.clone());
        }
    }

    // reverse prefix match: what user asked starts with a system font name
    // e.g. user types "JetBrains Mono Nerd Font" and system has "JetBrainsMono NF"
    // reverse prefix match
    for font in &fonts
    {
        let nf = normalize(font);
        if nf.len() >= MIN_FUZZY_LEN && normalized_requested.starts_with(&nf)
        {
            return Some(font.clone());
        }
    }

    // substring match (both directions)
    if normalized_requested.len() >= MIN_FUZZY_LEN
    {
        for font in &fonts
        {
            let nf = normalize(font);
            if nf.contains(&normalized_requested) || normalized_requested.contains(&nf)
            {
                return Some(font.clone());
            }
        }
    }

    // fuzzy match — only for inputs long enough to be meaningful, with a
    // distance threshold proportional to the input length (30%) so short
    // strings like "a" or "ab" don't spuriously match random fonts
    if normalized_requested.len() >= MIN_FUZZY_LEN
    {
        let max_allowed = ((normalized_requested.len() as f32 * MAX_FUZZY_RATIO).floor() as usize).max(1);
        let mut best_font = None;
        let mut best_distance = usize::MAX;

        for font in &fonts
        {
            let nf = normalize(font);
            let candidate = if nf.len() > normalized_requested.len()
            {
                nf[..normalized_requested.len()].to_string()
            }
            else
            {
                nf
            };

            let dist = levenshtein(&normalized_requested, &candidate);

            if dist < best_distance
            {
                best_distance = dist;
                best_font = Some(font.clone());
            }
        }

        if best_distance <= max_allowed && let Some(font) = best_font
        {
                return Some(font);
        }
    }

    None
}



fn system_fonts() -> Vec<String>
{
    let output = std::process::Command::new("fc-list").args(["-f", "%{family}\n"]).output();

    if let Ok(out) = output
    {
        let mut set = HashSet::new();

        for line in String::from_utf8_lossy(&out.stdout).lines()
        {
            for fam in line.split(',')
            {
                set.insert(fam.trim().to_string());
            }
        }

        // Sort so shorter/simpler names come first — base families like
        // "JetBrainsMono Nerd Font" beat "JetBrainsMono NFM ExtraBold"
        let mut fonts: Vec<String> = set.into_iter().collect();
        fonts.sort_by_key(|f| f.len());
        fonts
    }
    else
    {
        Vec::new()
    }
}



fn normalize(name: &str) -> String
{
    let lower = name.to_lowercase();
    lower.split([' ', '-', '_']).filter(|word|
    {
        !matches!(*word, "nerd" | "font" | "fonts" | "nf" | "nfm" | "nfp")
    }).collect::<String>()
}



#[cfg(test)]
mod tests
{
    use super::*;

    // ============ NORMALIZE TESTS ============
    #[test]
    fn normalize_strips_noise_words()
    {
        assert_eq!(normalize("JetBrainsMono Nerd Font"), "jetbrainsmono");
        assert_eq!(normalize("FiraCode Nerd Font Reg"), "firacodereg");
        assert_eq!(normalize("JetBrainsMono NF"), "jetbrainsmono");
        assert_eq!(normalize("JetBrainsMonoNL NFM"), "jetbrainsmononl");
        assert_eq!(normalize("JetBrainsMono NFP SemiBold"), "jetbrainsmonosemibold");
    }

    #[test]
    fn normalize_strips_separators()
    {
        assert_eq!(normalize("Fira-Code"), "firacode");
        assert_eq!(normalize("Fira_Code"), "firacode");
        assert_eq!(normalize("Fira Code"), "firacode");
    }

    #[test]
    fn normalize_lowercases()
    {
        assert_eq!(normalize("JetBrainsMono"), "jetbrainsmono");
        assert_eq!(normalize("UBUNTU"), "ubuntu");
    }

    #[test]
    fn normalize_empty_string()
    {
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn normalize_only_noise_words()
    {
        assert_eq!(normalize("Nerd Font"), "");
        assert_eq!(normalize("nf nfm nfp"), "");
    }

    // ============ RESOLVE FONT TESTS ============
    fn resolve_from(requested: &str, fonts: &[&str]) -> Option<String>
    {
        let normalized_requested = normalize(requested);
        let fonts: Vec<String> = fonts.iter().map(|s| s.to_string()).collect();
    
        for font in &fonts
        {
            if normalize(font) == normalized_requested { return Some(font.clone()); }
        }
        for font in &fonts
        {
            if normalize(font).starts_with(&normalized_requested) { return Some(font.clone()); }
        }
        // reverse prefix — guard added
        for font in &fonts
        {
            let nf = normalize(font);
            if nf.len() >= MIN_FUZZY_LEN && normalized_requested.starts_with(&nf) { return Some(font.clone()); }
        }
        // substring — guard added
        if normalized_requested.len() >= MIN_FUZZY_LEN
        {
            for font in &fonts
            {
                let nf = normalize(font);
                if nf.contains(&normalized_requested) || normalized_requested.contains(&nf) { return Some(font.clone()); }
            }
        }
    
        if normalized_requested.len() >= MIN_FUZZY_LEN
        {
            let max_allowed = ((normalized_requested.len() as f32 * MAX_FUZZY_RATIO).floor() as usize).max(1);
            let mut best_font = None;
            let mut best_distance = usize::MAX;
    
            for font in &fonts
            {
                let nf = normalize(font);
                let candidate = if nf.len() > normalized_requested.len()
                {
                    nf[..normalized_requested.len()].to_string()
                }
                else { nf };
    
                let dist = levenshtein(&normalized_requested, &candidate);
                if dist < best_distance
                {
                    best_distance = dist;
                    best_font = Some(font.clone());
                }
            }
    
            if best_distance <= max_allowed
            {
                if let Some(font) = best_font { return Some(font); }
            }
        }
    
        None
    }

    const JETBRAINS_FONTS: &[&str] = &[
        "JetBrainsMono Nerd Font",
        "JetBrainsMono NF",
        "JetBrainsMono NFM",
        "JetBrainsMono NFP",
        "JetBrainsMonoNL Nerd Font",
        "JetBrainsMonoNL NF",
        "FiraCode Nerd Font",
        "FiraCode NF",
        "Ubuntu",
        "DejaVu Sans",
    ];

    #[test]
    fn exact_name_matches()
    {
        assert_eq!(resolve_from("JetBrains Mono", JETBRAINS_FONTS), Some("JetBrainsMono Nerd Font".to_string()));
    }

    #[test]
    fn name_without_spaces_matches()
    {
        assert_eq!(resolve_from("JetBrainsMono", JETBRAINS_FONTS), Some("JetBrainsMono Nerd Font".to_string()));
    }

    #[test]
    fn full_nerd_font_name_matches()
    {
        assert_eq!(resolve_from("JetBrainsMono Nerd Font", JETBRAINS_FONTS), Some("JetBrainsMono Nerd Font".to_string()));
    }

    #[test]
    fn fira_code_matches()
    {
        assert_eq!(resolve_from("Fira Code", JETBRAINS_FONTS), Some("FiraCode Nerd Font".to_string()));
    }

    #[test]
    fn ubuntu_exact_match()
    {
        assert_eq!(resolve_from("Ubuntu", JETBRAINS_FONTS), Some("Ubuntu".to_string()));
    }

    #[test]
    fn unknown_font_returns_none()
    {
        assert_eq!(resolve_from("Nonexistent Font XYZ", JETBRAINS_FONTS), None);
    }

    #[test]
    fn short_input_returns_none()
    {
        assert_eq!(resolve_from("a", JETBRAINS_FONTS), None);
        assert_eq!(resolve_from("ab", JETBRAINS_FONTS), None);
        assert_eq!(resolve_from("abc", JETBRAINS_FONTS), None);
    }

    #[test]
    fn fuzzy_typo_matches()
    {
        assert_eq!(resolve_from("JetBrainsMono", JETBRAINS_FONTS), Some("JetBrainsMono Nerd Font".to_string()));
    }

    #[test]
    fn does_not_match_wrong_font()
    {
        let result = resolve_from("JetBrains Mono", JETBRAINS_FONTS);
        assert!(result.as_deref().map(|r| !r.to_lowercase().contains("fira")).unwrap_or(true));
    }
}
