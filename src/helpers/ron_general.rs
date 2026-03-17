// ============ CRATES ============
use crate::ron::BarConfig;





// ============ FUNCTIONS ============
pub fn apply_general_settings(ron_config: &mut BarConfig)
{
    if let Some(v) = ron_config.general_padding
    {
        ron_config.clock_padding                            = v;
        ron_config.media_player_metadata_padding            = v;
        ron_config.media_player_button_padding              = v;
        ron_config.focused_window_padding                   = v;
        ron_config.cpu_padding                              = v;
        ron_config.cpu_temp_padding                         = v;
        ron_config.ram_padding                              = v;
        ron_config.network_padding                          = v;
        ron_config.volume_output_padding                    = v;
        ron_config.volume_input_padding                     = v;
    }

    if let Some(v) = ron_config.general_text_size
    {
        ron_config.clock_text_size                          = v;
        ron_config.media_player_metadata_text_size          = v;
        ron_config.media_player_button_text_size            = v;
        ron_config.focused_window_text_size                 = v;
        ron_config.cpu_text_size                            = v;
        ron_config.cpu_temp_text_size                       = v;
        ron_config.ram_text_size                            = v;
        ron_config.network_text_size                        = v;
        ron_config.volume_output_text_size                  = v;
        ron_config.volume_input_text_size                   = v;
        ron_config.workspace_text_size                      = v;
    }

    if let Some(v) = ron_config.general_text_orientation
    {
        ron_config.clock_text_orientation                   = v;
        ron_config.media_player_metadata_text_orientation   = v;
        ron_config.media_player_button_text_orientation     = v;
        ron_config.focused_window_text_orientation          = v;
        ron_config.cpu_text_orientation                     = v;
        ron_config.cpu_temp_text_orientation                = v;
        ron_config.ram_text_orientation                     = v;
        ron_config.network_text_orientation                 = v;
        ron_config.volume_output_text_orientation           = v;
        ron_config.volume_input_text_orientation            = v;
        ron_config.workspace_text_orientation               = v;
    }

    if let Some(v) = ron_config.general_text_color
    {
        ron_config.clock_text_color                     = v;
        ron_config.media_player_metadata_text_color     = v;
        ron_config.focused_window_text_color            = v;
        ron_config.cpu_text_color                       = v;
        ron_config.cpu_temp_text_color                  = v;
        ron_config.ram_text_color                       = v;
        ron_config.network_text_color                   = v;
        ron_config.volume_output_text_color             = v;
        ron_config.volume_input_text_color              = v;
        ron_config.workspace_text_color                 = v;
    }

    if let Some(v) = ron_config.general_button_color
    {
        ron_config.clock_button_color                   = v;
        ron_config.tray_button_color                    = v;
        ron_config.media_player_metadata_button_color   = v;
        ron_config.media_player_button_color            = v;
        ron_config.focused_window_button_color          = v;
        ron_config.cpu_button_color                     = v;
        ron_config.cpu_temp_button_color                = v;
        ron_config.ram_button_color                     = v;
        ron_config.network_button_color                 = v;
        ron_config.volume_output_button_color           = v;
        ron_config.volume_input_button_color            = v;
        ron_config.workspace_button_color               = v;
    }

    if let Some(v) = ron_config.general_button_hovered_color
    {
        ron_config.clock_button_hovered_color                   = v;
        ron_config.tray_button_hovered_color                    = v;
        ron_config.media_player_metadata_button_hovered_color   = v;
        ron_config.media_player_button_hovered_color            = v;
        ron_config.focused_window_button_hovered_color          = v;
        ron_config.cpu_button_hovered_color                     = v;
        ron_config.cpu_temp_button_hovered_color                = v;
        ron_config.ram_button_hovered_color                     = v;
        ron_config.network_button_hovered_color                 = v;
        ron_config.volume_output_button_hovered_color           = v;
        ron_config.volume_input_button_hovered_color            = v;
        ron_config.workspace_button_hovered_color               = v;
    }

    if let Some(v) = ron_config.general_button_hovered_text_color
    {
        ron_config.clock_button_hovered_text_color                  = v;
        ron_config.tray_button_hovered_text_color                   = v;
        ron_config.media_player_metadata_button_hovered_text_color  = v;
        ron_config.media_player_button_hovered_text_color           = v;
        ron_config.focused_window_button_hovered_text_color         = v;
        ron_config.cpu_button_hovered_text_color                    = v;
        ron_config.cpu_temp_button_hovered_text_color               = v;
        ron_config.ram_button_hovered_text_color                    = v;
        ron_config.network_button_hovered_text_color                = v;
        ron_config.volume_output_button_hovered_text_color          = v;
        ron_config.volume_input_button_hovered_text_color           = v;
        ron_config.workspace_button_hovered_text_color              = v;
    }

    if let Some(v) = ron_config.general_button_pressed_color
    {
        ron_config.clock_button_pressed_color                   = v;
        ron_config.tray_button_pressed_color                    = v;
        ron_config.media_player_metadata_button_pressed_color   = v;
        ron_config.media_player_button_pressed_color            = v;
        ron_config.focused_window_button_pressed_color          = v;
        ron_config.cpu_button_pressed_color                     = v;
        ron_config.cpu_temp_button_pressed_color                = v;
        ron_config.ram_button_pressed_color                     = v;
        ron_config.network_button_pressed_color                 = v;
        ron_config.volume_output_button_pressed_color           = v;
        ron_config.volume_input_button_pressed_color            = v;
        ron_config.workspace_button_pressed_color               = v;
    }

    if let Some(v) = ron_config.general_border_color
    {
        ron_config.clock_border_color                  = v;
        ron_config.tray_border_color                   = v;
        ron_config.media_player_metadata_border_color  = v;
        ron_config.media_player_button_border_color    = v;
        ron_config.focused_window_border_color         = v;
        ron_config.cpu_border_color                    = v;
        ron_config.cpu_temp_border_color               = v;
        ron_config.ram_border_color                    = v;
        ron_config.network_border_color                = v;
        ron_config.volume_output_border_color          = v;
        ron_config.volume_input_border_color           = v;
        ron_config.workspace_border_color              = v;
    }

    if let Some(v) = ron_config.general_border_size
    {
        ron_config.clock_border_size                    = v;
        ron_config.tray_border_size                     = v;
        ron_config.media_player_metadata_border_size    = v;
        ron_config.media_player_button_border_size      = v;
        ron_config.focused_window_border_size           = v;
        ron_config.cpu_border_size                      = v;
        ron_config.cpu_temp_border_size                 = v;
        ron_config.ram_border_size                      = v;
        ron_config.network_border_size                  = v;
        ron_config.volume_output_border_size            = v;
        ron_config.volume_input_border_size             = v;
        ron_config.workspace_border_size                = v;
    }

    if let Some(v) = ron_config.general_border_radius
    {
        ron_config.clock_border_radius                  = v;
        ron_config.tray_border_radius                   = v;
        ron_config.media_player_metadata_border_radius  = v;
        ron_config.media_player_button_border_radius    = v;
        ron_config.focused_window_border_radius         = v;
        ron_config.cpu_border_radius                    = v;
        ron_config.cpu_temp_border_radius               = v;
        ron_config.ram_border_radius                    = v;
        ron_config.network_border_radius                = v;
        ron_config.volume_output_border_radius          = v;
        ron_config.volume_input_border_radius           = v;
        ron_config.workspace_border_radius              = v;
    }

    if let Some(v) = ron_config.general_side_separator
    {
        ron_config.clock_side_separator                     = Some(v);
        ron_config.tray_side_separator                      = Some(v);
        ron_config.media_player_metadata_side_separator     = Some(v);
        ron_config.media_player_buttons_side_separator      = Some(v);
        ron_config.focused_window_side_separator            = Some(v);
        ron_config.cpu_side_separator                       = Some(v);
        ron_config.cpu_temp_side_separator                  = Some(v);
        ron_config.ram_side_separator                       = Some(v);
        ron_config.network_side_separator                   = Some(v);
        ron_config.volume_output_side_separator             = Some(v);
        ron_config.volume_input_side_separator              = Some(v);
        ron_config.workspace_side_separator                 = Some(v);
    }

    if let Some(v) = ron_config.general_side_separator_color
    {
        ron_config.clock_side_separator_color                   = v;
        ron_config.tray_side_separator_color                    = v;
        ron_config.media_player_metadata_side_separator_color   = v;
        ron_config.media_player_buttons_side_separator_color    = v;
        ron_config.focused_window_side_separator_color          = v;
        ron_config.cpu_side_separator_color                     = v;
        ron_config.cpu_temp_side_separator_color                = v;
        ron_config.ram_side_separator_color                     = v;
        ron_config.network_side_separator_color                 = v;
        ron_config.volume_output_side_separator_color           = v;
        ron_config.volume_input_side_separator_color            = v;
        ron_config.workspace_side_separator_color               = v;
    }

    if let Some(v) = ron_config.general_side_separator_width
    {
        ron_config.clock_side_separator_width                   = v;
        ron_config.tray_side_separator_width                    = v;
        ron_config.media_player_metadata_side_separator_width   = v;
        ron_config.media_player_buttons_side_separator_width    = v;
        ron_config.focused_window_side_separator_width          = v;
        ron_config.cpu_side_separator_width                     = v;
        ron_config.cpu_temp_side_separator_width                = v;
        ron_config.ram_side_separator_width                     = v;
        ron_config.network_side_separator_width                 = v;
        ron_config.volume_output_side_separator_width           = v;
        ron_config.volume_input_side_separator_width            = v;
        ron_config.workspace_side_separator_width               = v;
    }

    if let Some(v) = ron_config.general_side_separator_height
    {
        ron_config.clock_side_separator_height                  = v;
        ron_config.tray_side_separator_height                   = v;
        ron_config.media_player_metadata_side_separator_height  = v;
        ron_config.media_player_buttons_side_separator_height   = v;
        ron_config.focused_window_side_separator_height         = v;
        ron_config.cpu_side_separator_height                    = v;
        ron_config.cpu_temp_side_separator_height               = v;
        ron_config.ram_side_separator_height                    = v;
        ron_config.network_side_separator_height                = v;
        ron_config.volume_output_side_separator_height          = v;
        ron_config.volume_input_side_separator_height           = v;
        ron_config.workspace_side_separator_height              = v;
    }


    // ---- alt fields ----

    if let Some(v) = ron_config.general_alt_padding
    {
        ron_config.alt_clock_padding            = v;
        ron_config.alt_network_padding          = v;
        ron_config.muted_volume_output_padding  = v;
        ron_config.muted_volume_input_padding   = v;
    }

    if let Some(v) = ron_config.general_alt_text_size
    {
        ron_config.alt_clock_text_size          = v;
        ron_config.alt_network_text_size        = v;
        ron_config.muted_volume_output_text_size = v;
        ron_config.muted_volume_input_text_size = v;
    }

    if let Some(v) = ron_config.general_alt_text_orientation
    {
        ron_config.alt_clock_text_orientation           = v;
        ron_config.alt_network_text_orientation         = v;
        ron_config.muted_volume_output_text_orientation = v;
        ron_config.muted_volume_input_text_orientation  = v;
    }

    if let Some(v) = ron_config.general_alt_text_color
    {
        ron_config.alt_clock_text_color             = v;
        ron_config.alt_network_text_color           = v;
        ron_config.muted_volume_output_text_color   = v;
        ron_config.muted_volume_input_text_color    = v;
    }

    if let Some(v) = ron_config.general_alt_button_color
    {
        ron_config.alt_clock_button_color               = v;
        ron_config.alt_network_button_color             = v;
        ron_config.muted_volume_output_button_color     = v;
        ron_config.muted_volume_input_button_color      = v;
    }

    if let Some(v) = ron_config.general_alt_button_hovered_color
    {
        ron_config.alt_clock_button_hovered_color               = v;
        ron_config.alt_network_button_hovered_color             = v;
        ron_config.muted_volume_output_button_hovered_color     = v;
        ron_config.muted_volume_input_button_hovered_color      = v;
    }

    if let Some(v) = ron_config.general_alt_button_hovered_text_color
    {
        ron_config.alt_clock_button_hovered_text_color              = v;
        ron_config.alt_network_button_hovered_text_color            = v;
        ron_config.muted_volume_output_button_hovered_text_color    = v;
        ron_config.muted_volume_input_button_hovered_text_color     = v;
    }

    if let Some(v) = ron_config.general_alt_button_pressed_color
    {
        ron_config.alt_clock_button_pressed_color               = v;
        ron_config.alt_network_button_pressed_color             = v;
        ron_config.muted_volume_output_button_pressed_color     = v;
        ron_config.muted_volume_input_button_pressed_color      = v;
    }

    if let Some(v) = ron_config.general_alt_border_color
    {
        ron_config.alt_clock_border_color              = v;
        ron_config.alt_network_border_color            = v;
        ron_config.muted_volume_output_border_color    = v;
        ron_config.muted_volume_input_border_color     = v;
    }

    if let Some(v) = ron_config.general_alt_border_size
    {
        ron_config.alt_clock_border_size            = v;
        ron_config.alt_network_border_size          = v;
        ron_config.muted_volume_output_border_size  = v;
        ron_config.muted_volume_input_border_size   = v;
    }

    if let Some(v) = ron_config.general_alt_border_radius
    {
        ron_config.alt_clock_border_radius              = v;
        ron_config.alt_network_border_radius            = v;
        ron_config.muted_volume_output_border_radius    = v;
        ron_config.muted_volume_input_border_radius     = v;
    }
    
    if let Some(v) = ron_config.general_alt_side_separator
    {
        ron_config.alt_clock_side_separator             = Some(v);
        ron_config.alt_network_side_separator               = Some(v);
        ron_config.muted_volume_output_side_separator   = Some(v);
        ron_config.muted_volume_input_side_separator    = Some(v);
    }

    if let Some(v) = ron_config.general_alt_side_separator_color
    {
        ron_config.alt_clock_side_separator_color           = v;
        ron_config.alt_network_side_separator_color         = v; 
        ron_config.muted_volume_output_side_separator_color = v;
        ron_config.muted_volume_input_side_separator_color  = v;
    }

    if let Some(v) = ron_config.general_alt_side_separator_width
    {
        ron_config.alt_clock_side_separator_width           = v;
        ron_config.alt_network_side_separator_width         = v; 
        ron_config.muted_volume_output_side_separator_width = v;
        ron_config.muted_volume_input_side_separator_width  = v;
    }

    if let Some(v) = ron_config.general_alt_side_separator_height
    {
        ron_config.alt_clock_side_separator_height              = v;
        ron_config.alt_network_side_separator_height            = v; 
        ron_config.muted_volume_output_side_separator_height    = v;
        ron_config.muted_volume_input_side_separator_height     = v;
    }
}
