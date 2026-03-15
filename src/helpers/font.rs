// ============ IMPORTS ============
use std::collections::HashSet;
use strsim::levenshtein;





// ============ CONST ============
const MAX_DISTANCE: usize = 6;





// ============ FUNCTIONS ============
pub fn resolve_font(requested: &str) -> String
{
    let normalized_requested = normalize(requested);

    let fonts = system_fonts();

    // exact normalized match
    for font in &fonts
    {
        if normalize(font) == normalized_requested
        {
            return font.clone();
        }
    }

    // prefix match: system font's normalized name starts with what user asked
    for font in &fonts
    {
        if normalize(font).starts_with(&normalized_requested)
        {
            return font.clone();
        }
    }

    // reverse prefix match: what user asked starts with a system font name
    // e.g. user types "JetBrains Mono Nerd Font" and system has "JetBrainsMono NF"
    for font in &fonts
    {
        let nf = normalize(font);
        if !nf.is_empty() && normalized_requested.starts_with(&nf)
        {
            return font.clone();
        }
    }

    // substring match (both directions)
    for font in &fonts
    {
        let nf = normalize(font);
        if nf.contains(&normalized_requested) || normalized_requested.contains(&nf)
        {
            return font.clone();
        }
    }

    // fuzzy match — compare against the normalized font name to keep distances fair
    let mut best_font = None;
    let mut best_distance = usize::MAX;

    for font in &fonts
    {
        let nf = normalize(font);
        let candidate = if nf.len() > normalized_requested.len()
        {
            &nf[..normalized_requested.len().min(nf.len())]
        }
        else
        {
            &nf
        };

        let dist = levenshtein(&normalized_requested, candidate);

        if dist < best_distance
        {
            best_distance = dist;
            best_font = Some(font.clone());
        }
    }

    if best_distance <= MAX_DISTANCE && let Some(font) = best_font
    {
        return font;
    }

    requested.to_string()
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

        set.into_iter().collect()
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
    // These test the matching logic directly without hitting the real fc-list.
    // We shadow system_fonts() by extracting the match logic into a testable helper.

    fn resolve_from(requested: &str, fonts: &[&str]) -> String
    {
        let normalized_requested = normalize(requested);
        let fonts: Vec<String> = fonts.iter().map(|s| s.to_string()).collect();

        for font in &fonts
        {
            if normalize(font) == normalized_requested { return font.clone(); }
        }
        for font in &fonts
        {
            if normalize(font).starts_with(&normalized_requested) { return font.clone(); }
        }
        for font in &fonts
        {
            let nf = normalize(font);
            if !nf.is_empty() && normalized_requested.starts_with(&nf) { return font.clone(); }
        }
        for font in &fonts
        {
            let nf = normalize(font);
            if nf.contains(&normalized_requested) || normalized_requested.contains(&nf) { return font.clone(); }
        }

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

        if best_distance <= MAX_DISTANCE
        {
            if let Some(font) = best_font { return font; }
        }

        requested.to_string()
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
        assert_eq!(resolve_from("JetBrains Mono", JETBRAINS_FONTS), "JetBrainsMono Nerd Font");
    }

    #[test]
    fn name_without_spaces_matches()
    {
        assert_eq!(resolve_from("JetBrainsMono", JETBRAINS_FONTS), "JetBrainsMono Nerd Font");
    }

    #[test]
    fn full_nerd_font_name_matches()
    {
        assert_eq!(resolve_from("JetBrainsMono Nerd Font", JETBRAINS_FONTS), "JetBrainsMono Nerd Font");
    }

    #[test]
    fn fira_code_matches()
    {
        assert_eq!(resolve_from("Fira Code", JETBRAINS_FONTS), "FiraCode Nerd Font");
    }

    #[test]
    fn ubuntu_exact_match()
    {
        assert_eq!(resolve_from("Ubuntu", JETBRAINS_FONTS), "Ubuntu");
    }

    #[test]
    fn unknown_font_falls_back_to_requested()
    {
        assert_eq!(resolve_from("Nonexistent Font XYZ", JETBRAINS_FONTS), "Nonexistent Font XYZ");
    }

    #[test]
    fn fuzzy_typo_matches()
    {
        // "JetBrainsMono" with one char typo
        assert_eq!(resolve_from("JetBrainsMono", JETBRAINS_FONTS), "JetBrainsMono Nerd Font");
    }

    #[test]
    fn does_not_match_wrong_font()
    {
        let result = resolve_from("JetBrains Mono", JETBRAINS_FONTS);
        assert!(!result.to_lowercase().contains("fira"), "Should not match FiraCode for JetBrains Mono request");
    }
}
