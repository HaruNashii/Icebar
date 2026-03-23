// ============ CRATES ============
use crate::ron::BarConfig;





// ============ FUNCTIONS ============
pub fn apply_general_settings(ron_config: &mut BarConfig)
{
    if let Some(v) = ron_config.general_style.general_padding
    {
        ron_config.clock.clock_padding                                              = v;
        ron_config.media_player_metadata.media_player_metadata_padding              = v;
        ron_config.media_player_button.media_player_button_padding                  = v;
        ron_config.focused_window.focused_window_padding                            = v;
        ron_config.cpu.cpu_padding                                                  = v;
        ron_config.cpu_temp.cpu_temp_padding                                        = v;
        ron_config.ram.ram_padding                                                  = v;
        ron_config.network.network_padding                                          = v;
        ron_config.volume_output.volume_output_padding                              = v;
        ron_config.volume_input.volume_input_padding                                = v;
        ron_config.disk.disk_padding                                                = v;
    }

    if let Some(v) = ron_config.general_style.general_text_size
    {
        ron_config.clock.clock_text_size                                            = v;
        ron_config.media_player_metadata.media_player_metadata_text_size            = v;
        ron_config.media_player_button.media_player_button_text_size                = v;
        ron_config.focused_window.focused_window_text_size                          = v;
        ron_config.cpu.cpu_text_size                                                = v;
        ron_config.cpu_temp.cpu_temp_text_size                                      = v;
        ron_config.ram.ram_text_size                                                = v;
        ron_config.network.network_text_size                                        = v;
        ron_config.volume_output.volume_output_text_size                            = v;
        ron_config.volume_input.volume_input_text_size                              = v;
        ron_config.workspace.workspace_text_size                                    = v;
        ron_config.disk.disk_text_size                                              = v;
    }

    if let Some(v) = ron_config.general_style.general_text_orientation
    {
        ron_config.clock.clock_text_orientation                                     = v;
        ron_config.media_player_metadata.media_player_metadata_text_orientation     = v;
        ron_config.media_player_button.media_player_button_text_orientation         = v;
        ron_config.focused_window.focused_window_text_orientation                   = v;
        ron_config.cpu.cpu_text_orientation                                         = v;
        ron_config.cpu_temp.cpu_temp_text_orientation                               = v;
        ron_config.ram.ram_text_orientation                                         = v;
        ron_config.network.network_text_orientation                                 = v;
        ron_config.volume_output.volume_output_text_orientation                     = v;
        ron_config.volume_input.volume_input_text_orientation                       = v;
        ron_config.workspace.workspace_text_orientation                             = v;
        ron_config.disk.disk_text_orientation                                       = v;
    }

    if let Some(v) = ron_config.general_style.general_text_color
    {
        ron_config.clock.clock_text_color                                           = v;
        ron_config.media_player_metadata.media_player_metadata_text_color           = v;
        ron_config.media_player_button.media_player_button_text_color               = v;
        ron_config.focused_window.focused_window_text_color                         = v;
        ron_config.cpu.cpu_text_color                                               = v;
        ron_config.cpu_temp.cpu_temp_text_color                                     = v;
        ron_config.ram.ram_text_color                                               = v;
        ron_config.network.network_text_color                                       = v;
        ron_config.volume_output.volume_output_text_color                           = v;
        ron_config.volume_input.volume_input_text_color                             = v;
        ron_config.workspace.workspace_text_color                                   = v;
        ron_config.disk.disk_text_color                                             = v;
    }

    if let Some(v) = ron_config.general_style.general_button_color
    {
        ron_config.clock.clock_button_color                   = v;
        ron_config.tray.tray_button_color                    = v;
        ron_config.media_player_metadata.media_player_metadata_button_color   = v;
        ron_config.media_player_button.media_player_button_color            = v;
        ron_config.focused_window.focused_window_button_color          = v;
        ron_config.cpu.cpu_button_color                     = v;
        ron_config.cpu_temp.cpu_temp_button_color                = v;
        ron_config.ram.ram_button_color                     = v;
        ron_config.network.network_button_color                 = v;
        ron_config.volume_output.volume_output_button_color           = v;
        ron_config.volume_input.volume_input_button_color            = v;
        ron_config.workspace.workspace_button_color               = v;
        ron_config.disk.disk_button_color                    = v;
    }

    if let Some(v) = ron_config.general_style.general_button_hovered_color
    {
        ron_config.clock.clock_button_hovered_color                   = v;
        ron_config.tray.tray_button_hovered_color                    = v;
        ron_config.media_player_metadata.media_player_metadata_button_hovered_color   = v;
        ron_config.media_player_button.media_player_button_hovered_color            = v;
        ron_config.focused_window.focused_window_button_hovered_color          = v;
        ron_config.cpu.cpu_button_hovered_color                     = v;
        ron_config.cpu_temp.cpu_temp_button_hovered_color                = v;
        ron_config.ram.ram_button_hovered_color                     = v;
        ron_config.network.network_button_hovered_color                 = v;
        ron_config.volume_output.volume_output_button_hovered_color           = v;
        ron_config.volume_input.volume_input_button_hovered_color            = v;
        ron_config.workspace.workspace_button_hovered_color               = v;
        ron_config.disk.disk_button_hovered_color                    = v;
    }

    if let Some(v) = ron_config.general_style.general_button_hovered_text_color
    {
        ron_config.clock.clock_button_hovered_text_color                  = v;
        ron_config.tray.tray_button_hovered_text_color                   = v;
        ron_config.media_player_metadata.media_player_metadata_button_hovered_text_color  = v;
        ron_config.media_player_button.media_player_button_hovered_text_color           = v;
        ron_config.focused_window.focused_window_button_hovered_text_color         = v;
        ron_config.cpu.cpu_button_hovered_text_color                    = v;
        ron_config.cpu_temp.cpu_temp_button_hovered_text_color               = v;
        ron_config.ram.ram_button_hovered_text_color                    = v;
        ron_config.network.network_button_hovered_text_color                = v;
        ron_config.volume_output.volume_output_button_hovered_text_color          = v;
        ron_config.volume_input.volume_input_button_hovered_text_color           = v;
        ron_config.workspace.workspace_button_hovered_text_color              = v;
        ron_config.disk.disk_button_hovered_text_color                   = v;
    }

    if let Some(v) = ron_config.general_style.general_button_pressed_text_color
    {
        ron_config.clock.clock_button_pressed_text_color                  = v;
        ron_config.tray.tray_button_pressed_text_color                   = v;
        ron_config.media_player_metadata.media_player_metadata_button_pressed_text_color  = v;
        ron_config.media_player_button.media_player_button_pressed_text_color           = v;
        ron_config.focused_window.focused_window_button_pressed_text_color         = v;
        ron_config.cpu.cpu_button_pressed_text_color                    = v;
        ron_config.cpu_temp.cpu_temp_button_pressed_text_color               = v;
        ron_config.ram.ram_button_pressed_text_color                    = v;
        ron_config.network.network_button_pressed_text_color                = v;
        ron_config.volume_output.volume_output_button_pressed_text_color          = v;
        ron_config.volume_input.volume_input_button_pressed_text_color           = v;
        ron_config.workspace.workspace_button_pressed_text_color              = v;
        ron_config.disk.disk_button_pressed_text_color                   = v;
    }

    if let Some(v) = ron_config.general_style.general_button_pressed_color
    {
        ron_config.clock.clock_button_pressed_color                   = v;
        ron_config.tray.tray_button_pressed_color                    = v;
        ron_config.media_player_metadata.media_player_metadata_button_pressed_color   = v;
        ron_config.media_player_button.media_player_button_pressed_color            = v;
        ron_config.focused_window.focused_window_button_pressed_color          = v;
        ron_config.cpu.cpu_button_pressed_color                     = v;
        ron_config.cpu_temp.cpu_temp_button_pressed_color                = v;
        ron_config.ram.ram_button_pressed_color                     = v;
        ron_config.network.network_button_pressed_color                 = v;
        ron_config.volume_output.volume_output_button_pressed_color           = v;
        ron_config.volume_input.volume_input_button_pressed_color            = v;
        ron_config.workspace.workspace_button_pressed_color               = v;
        ron_config.disk.disk_button_pressed_color                    = v;
    }

    if let Some(v) = ron_config.general_style.general_border_color
    {
        ron_config.clock.clock_border_color                  = v;
        ron_config.tray.tray_border_color                   = v;
        ron_config.media_player_metadata.media_player_metadata_border_color  = v;
        ron_config.media_player_button.media_player_button_border_color    = v;
        ron_config.focused_window.focused_window_border_color         = v;
        ron_config.cpu.cpu_border_color                    = v;
        ron_config.cpu_temp.cpu_temp_border_color               = v;
        ron_config.ram.ram_border_color                    = v;
        ron_config.network.network_border_color                = v;
        ron_config.volume_output.volume_output_border_color          = v;
        ron_config.volume_input.volume_input_border_color           = v;
        ron_config.workspace.workspace_border_color              = v;
        ron_config.disk.disk_border_color                   = v;
    }

    if let Some(v) = ron_config.general_style.general_border_size
    {
        ron_config.clock.clock_border_size                    = v;
        ron_config.tray.tray_border_size                     = v;
        ron_config.media_player_metadata.media_player_metadata_border_size    = v;
        ron_config.media_player_button.media_player_button_border_size      = v;
        ron_config.focused_window.focused_window_border_size           = v;
        ron_config.cpu.cpu_border_size                      = v;
        ron_config.cpu_temp.cpu_temp_border_size                 = v;
        ron_config.ram.ram_border_size                      = v;
        ron_config.network.network_border_size                  = v;
        ron_config.volume_output.volume_output_border_size            = v;
        ron_config.volume_input.volume_input_border_size             = v;
        ron_config.workspace.workspace_border_size                = v;
        ron_config.disk.disk_border_size                     = v;
    }

    if let Some(v) = ron_config.general_style.general_border_radius
    {
        ron_config.clock.clock_border_radius                  = v;
        ron_config.tray.tray_border_radius                   = v;
        ron_config.media_player_metadata.media_player_metadata_border_radius  = v;
        ron_config.media_player_button.media_player_button_border_radius    = v;
        ron_config.focused_window.focused_window_border_radius         = v;
        ron_config.cpu.cpu_border_radius                    = v;
        ron_config.cpu_temp.cpu_temp_border_radius               = v;
        ron_config.ram.ram_border_radius                    = v;
        ron_config.network.network_border_radius                = v;
        ron_config.volume_output.volume_output_border_radius          = v;
        ron_config.volume_input.volume_input_border_radius           = v;
        ron_config.workspace.workspace_border_radius              = v;
        ron_config.disk.disk_border_radius                    = v;
    }

    if let Some(v) = ron_config.general_style.general_side_separator
    {
        ron_config.clock.clock_side_separator                     = Some(v);
        ron_config.tray.tray_side_separator                      = Some(v);
        ron_config.media_player_metadata.media_player_metadata_side_separator     = Some(v);
        ron_config.media_player_button.media_player_buttons_side_separator      = Some(v);
        ron_config.focused_window.focused_window_side_separator            = Some(v);
        ron_config.cpu.cpu_side_separator                       = Some(v);
        ron_config.cpu_temp.cpu_temp_side_separator                  = Some(v);
        ron_config.ram.ram_side_separator                       = Some(v);
        ron_config.network.network_side_separator                   = Some(v);
        ron_config.volume_output.volume_output_side_separator             = Some(v);
        ron_config.volume_input.volume_input_side_separator              = Some(v);
        ron_config.workspace.workspace_side_separator                 = Some(v);
        ron_config.disk.disk_side_separator                       = Some(v);
    }

    if let Some(v) = ron_config.general_style.general_side_separator_color
    {
        ron_config.clock.clock_side_separator_color                   = v;
        ron_config.tray.tray_side_separator_color                    = v;
        ron_config.media_player_metadata.media_player_metadata_side_separator_color   = v;
        ron_config.media_player_button.media_player_buttons_side_separator_color    = v;
        ron_config.focused_window.focused_window_side_separator_color          = v;
        ron_config.cpu.cpu_side_separator_color                     = v;
        ron_config.cpu_temp.cpu_temp_side_separator_color                = v;
        ron_config.ram.ram_side_separator_color                     = v;
        ron_config.network.network_side_separator_color                 = v;
        ron_config.volume_output.volume_output_side_separator_color           = v;
        ron_config.volume_input.volume_input_side_separator_color            = v;
        ron_config.workspace.workspace_side_separator_color               = v;
        ron_config.disk.disk_side_separator_color                     = v;
    }

    if let Some(v) = ron_config.general_style.general_side_separator_width
    {
        ron_config.clock.clock_side_separator_width                   = v;
        ron_config.tray.tray_side_separator_width                    = v;
        ron_config.media_player_metadata.media_player_metadata_side_separator_width   = v;
        ron_config.media_player_button.media_player_buttons_side_separator_width    = v;
        ron_config.focused_window.focused_window_side_separator_width          = v;
        ron_config.cpu.cpu_side_separator_width                     = v;
        ron_config.cpu_temp.cpu_temp_side_separator_width                = v;
        ron_config.ram.ram_side_separator_width                     = v;
        ron_config.network.network_side_separator_width                 = v;
        ron_config.volume_output.volume_output_side_separator_width           = v;
        ron_config.volume_input.volume_input_side_separator_width            = v;
        ron_config.workspace.workspace_side_separator_width               = v;
        ron_config.disk.disk_side_separator_width                     = v;
    }

    if let Some(v) = ron_config.general_style.general_side_separator_height
    {
        ron_config.clock.clock_side_separator_height                  = v;
        ron_config.tray.tray_side_separator_height                   = v;
        ron_config.media_player_metadata.media_player_metadata_side_separator_height  = v;
        ron_config.media_player_button.media_player_buttons_side_separator_height   = v;
        ron_config.focused_window.focused_window_side_separator_height         = v;
        ron_config.cpu.cpu_side_separator_height                    = v;
        ron_config.cpu_temp.cpu_temp_side_separator_height               = v;
        ron_config.ram.ram_side_separator_height                    = v;
        ron_config.network.network_side_separator_height                = v;
        ron_config.volume_output.volume_output_side_separator_height          = v;
        ron_config.volume_input.volume_input_side_separator_height           = v;
        ron_config.workspace.workspace_side_separator_height              = v;
        ron_config.disk.disk_side_separator_height                    = v;
    }


    // ---- alt fields ----

    if let Some(v) = ron_config.general_style.general_alt_padding
    {
        ron_config.clock.alt_clock_padding            = v;
        ron_config.alt_network.alt_network_padding          = v;
        ron_config.muted_volume_output.muted_volume_output_padding  = v;
        ron_config.muted_volume_input.muted_volume_input_padding   = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_text_size
    {
        ron_config.clock.alt_clock_text_size          = v;
        ron_config.alt_network.alt_network_text_size        = v;
        ron_config.muted_volume_output.muted_volume_output_text_size = v;
        ron_config.muted_volume_input.muted_volume_input_text_size = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_text_orientation
    {
        ron_config.clock.alt_clock_text_orientation           = v;
        ron_config.alt_network.alt_network_text_orientation         = v;
        ron_config.muted_volume_output.muted_volume_output_text_orientation = v;
        ron_config.muted_volume_input.muted_volume_input_text_orientation  = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_text_color
    {
        ron_config.clock.alt_clock_text_color             = v;
        ron_config.alt_network.alt_network_text_color           = v;
        ron_config.muted_volume_output.muted_volume_output_text_color   = v;
        ron_config.muted_volume_input.muted_volume_input_text_color    = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_button_color
    {
        ron_config.clock.alt_clock_button_color               = v;
        ron_config.alt_network.alt_network_button_color             = v;
        ron_config.muted_volume_output.muted_volume_output_button_color     = v;
        ron_config.muted_volume_input.muted_volume_input_button_color      = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_button_hovered_color
    {
        ron_config.clock.alt_clock_button_hovered_color               = v;
        ron_config.alt_network.alt_network_button_hovered_color             = v;
        ron_config.muted_volume_output.muted_volume_output_button_hovered_color     = v;
        ron_config.muted_volume_input.muted_volume_input_button_hovered_color      = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_button_hovered_text_color
    {
        ron_config.clock.alt_clock_button_hovered_text_color              = v;
        ron_config.alt_network.alt_network_button_hovered_text_color            = v;
        ron_config.muted_volume_output.muted_volume_output_button_hovered_text_color    = v;
        ron_config.muted_volume_input.muted_volume_input_button_hovered_text_color     = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_button_pressed_text_color
    {
        ron_config.clock.alt_clock_button_pressed_text_color              = v;
        ron_config.alt_network.alt_network_button_pressed_text_color            = v;
        ron_config.muted_volume_output.muted_volume_output_button_pressed_text_color    = v;
        ron_config.muted_volume_input.muted_volume_input_button_pressed_text_color     = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_button_pressed_color
    {
        ron_config.clock.alt_clock_button_pressed_color               = v;
        ron_config.alt_network.alt_network_button_pressed_color             = v;
        ron_config.muted_volume_output.muted_volume_output_button_pressed_color     = v;
        ron_config.muted_volume_input.muted_volume_input_button_pressed_color      = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_border_color
    {
        ron_config.clock.alt_clock_border_color              = v;
        ron_config.alt_network.alt_network_border_color            = v;
        ron_config.muted_volume_output.muted_volume_output_border_color    = v;
        ron_config.muted_volume_input.muted_volume_input_border_color     = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_border_size
    {
        ron_config.clock.alt_clock_border_size            = v;
        ron_config.alt_network.alt_network_border_size          = v;
        ron_config.muted_volume_output.muted_volume_output_border_size  = v;
        ron_config.muted_volume_input.muted_volume_input_border_size   = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_border_radius
    {
        ron_config.clock.alt_clock_border_radius              = v;
        ron_config.alt_network.alt_network_border_radius            = v;
        ron_config.muted_volume_output.muted_volume_output_border_radius    = v;
        ron_config.muted_volume_input.muted_volume_input_border_radius     = v;
    }
    
    if let Some(v) = ron_config.general_style.general_alt_side_separator
    {
        ron_config.clock.alt_clock_side_separator             = Some(v);
        ron_config.alt_network.alt_network_side_separator               = Some(v);
        ron_config.muted_volume_output.muted_volume_output_side_separator   = Some(v);
        ron_config.muted_volume_input.muted_volume_input_side_separator    = Some(v);
    }

    if let Some(v) = ron_config.general_style.general_alt_side_separator_color
    {
        ron_config.clock.alt_clock_side_separator_color           = v;
        ron_config.alt_network.alt_network_side_separator_color         = v; 
        ron_config.muted_volume_output.muted_volume_output_side_separator_color = v;
        ron_config.muted_volume_input.muted_volume_input_side_separator_color  = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_side_separator_width
    {
        ron_config.clock.alt_clock_side_separator_width           = v;
        ron_config.alt_network.alt_network_side_separator_width         = v; 
        ron_config.muted_volume_output.muted_volume_output_side_separator_width = v;
        ron_config.muted_volume_input.muted_volume_input_side_separator_width  = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_side_separator_height
    {
        ron_config.clock.alt_clock_side_separator_height              = v;
        ron_config.alt_network.alt_network_side_separator_height            = v; 
        ron_config.muted_volume_output.muted_volume_output_side_separator_height    = v;
        ron_config.muted_volume_input.muted_volume_input_side_separator_height     = v;
    }

    if let Some(v) = &ron_config.general_style.general_button_gradient_color
    {
        ron_config.disk.disk_button_gradient_color                                  = Some(v.clone());
        ron_config.tray.tray_button_gradient_color                                  = Some(v.clone());
        ron_config.focused_window.focused_window_button_gradient_color                        = Some(v.clone());
        ron_config.cpu.cpu_button_gradient_color                                   = Some(v.clone());
        ron_config.cpu_temp.cpu_temp_button_gradient_color                              = Some(v.clone());
        ron_config.media_player_metadata.media_player_metadata_button_gradient_color                 = Some(v.clone());
        ron_config.media_player_button.media_player_button_gradient_color                          = Some(v.clone());
        ron_config.network.network_button_gradient_color                               = Some(v.clone());
        ron_config.ram.ram_button_gradient_color                                   = Some(v.clone());
        ron_config.volume_output.volume_output_button_gradient_color                         = Some(v.clone());
        ron_config.clock.clock_button_gradient_color                                 = Some(v.clone());
        ron_config.volume_input.volume_input_button_gradient_color                          = Some(v.clone());
        ron_config.workspace.workspace_button_gradient_color                             = Some(v.clone());
    }

    if let Some(v) = &ron_config.general_style.general_button_hovered_gradient_color
    {
        ron_config.cpu.cpu_button_hovered_gradient_color                           = Some(v.clone());
        ron_config.disk.disk_button_hovered_gradient_color                          = Some(v.clone());
        ron_config.cpu_temp.cpu_temp_button_hovered_gradient_color                      = Some(v.clone());
        ron_config.tray.tray_button_hovered_gradient_color                          = Some(v.clone());
        ron_config.media_player_button.media_player_button_hovered_gradient_color                  = Some(v.clone());
        ron_config.media_player_metadata.media_player_metadata_button_hovered_gradient_color         = Some(v.clone());
        ron_config.ram.ram_button_hovered_gradient_color                           = Some(v.clone());
        ron_config.focused_window.focused_window_button_hovered_gradient_color                = Some(v.clone());
        ron_config.clock.clock_button_hovered_gradient_color                         = Some(v.clone());
        ron_config.network.network_button_hovered_gradient_color                       = Some(v.clone());
        ron_config.volume_input.volume_input_button_hovered_gradient_color                  = Some(v.clone());
        ron_config.workspace.workspace_button_hovered_gradient_color                     = Some(v.clone());
        ron_config.volume_output.volume_output_button_hovered_gradient_color                 = Some(v.clone());
    }

    if let Some(v) = &ron_config.general_style.general_button_pressed_gradient_color
    {
        ron_config.cpu_temp.cpu_temp_button_pressed_gradient_color                      = Some(v.clone());
        ron_config.disk.disk_button_pressed_gradient_color                          = Some(v.clone());
        ron_config.focused_window.focused_window_button_pressed_gradient_color                = Some(v.clone());
        ron_config.tray.tray_button_pressed_gradient_color                          = Some(v.clone());
        ron_config.ram.ram_button_pressed_gradient_color                           = Some(v.clone());
        ron_config.cpu.cpu_button_pressed_gradient_color                           = Some(v.clone());
        ron_config.media_player_metadata.media_player_metadata_button_pressed_gradient_color         = Some(v.clone());
        ron_config.media_player_button.media_player_button_pressed_gradient_color                  = Some(v.clone());
        ron_config.network.network_button_pressed_gradient_color                       = Some(v.clone());
        ron_config.clock.clock_button_pressed_gradient_color                         = Some(v.clone());
        ron_config.volume_output.volume_output_button_pressed_gradient_color                 = Some(v.clone());
        ron_config.volume_input.volume_input_button_pressed_gradient_color                  = Some(v.clone());
        ron_config.workspace.workspace_button_pressed_gradient_color                     = Some(v.clone());
    }

    if let Some(v) = &ron_config.general_style.general_alt_button_gradient_color
    {
        ron_config.muted_volume_input.muted_volume_input_button_gradient_color                    = Some(v.clone());
        ron_config.muted_volume_output.muted_volume_output_button_gradient_color                   = Some(v.clone());
        ron_config.alt_network.alt_network_button_gradient_color                           = Some(v.clone());
        ron_config.clock.alt_clock_button_gradient_color                             = Some(v.clone());
    }

    if let Some(v) = &ron_config.general_style.general_alt_button_hovered_gradient_color
    {
        ron_config.muted_volume_input.muted_volume_input_button_hovered_gradient_color            = Some(v.clone());
        ron_config.muted_volume_output.muted_volume_output_button_hovered_gradient_color           = Some(v.clone());
        ron_config.alt_network.alt_network_button_hovered_gradient_color                   = Some(v.clone());
        ron_config.clock.alt_clock_button_hovered_gradient_color                     = Some(v.clone());
    }

    if let Some(v) = &ron_config.general_style.general_alt_button_pressed_gradient_color
    {
        ron_config.muted_volume_output.muted_volume_output_button_pressed_gradient_color           = Some(v.clone());
        ron_config.muted_volume_input.muted_volume_input_button_pressed_gradient_color            = Some(v.clone());
        ron_config.alt_network.alt_network_button_pressed_gradient_color                         = Some(v.clone());
        ron_config.clock.alt_clock_button_pressed_gradient_color                                = Some(v.clone());
    }

    if let Some(v) = &ron_config.general_style.general_alt_button_shadow_color
    {
        ron_config.muted_volume_input.muted_volume_input_button_shadow_color                  = Some(*v);
        ron_config.muted_volume_output.muted_volume_output_button_shadow_color                = Some(*v);
        ron_config.alt_network.alt_network_button_shadow_color                                = Some(*v);
        ron_config.clock.alt_clock_button_shadow_color                                        = Some(*v);
    }

    if let Some(v) = ron_config.general_style.general_alt_button_shadow_x
    {
        ron_config.muted_volume_input.muted_volume_input_button_shadow_x                  = v;
        ron_config.muted_volume_output.muted_volume_output_button_shadow_x                = v;
        ron_config.alt_network.alt_network_button_shadow_x                                = v;
        ron_config.clock.alt_clock_button_shadow_x                                        = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_button_shadow_y
    {
        ron_config.muted_volume_input.muted_volume_input_button_shadow_y                  = v;
        ron_config.muted_volume_output.muted_volume_output_button_shadow_y                = v;
        ron_config.alt_network.alt_network_button_shadow_y                                = v;
        ron_config.clock.alt_clock_button_shadow_y                                        = v;
    }

    if let Some(v) = ron_config.general_style.general_alt_button_shadow_blur
    {
        ron_config.muted_volume_input.muted_volume_input_button_shadow_blur                  = v;
        ron_config.muted_volume_output.muted_volume_output_button_shadow_blur                = v;
        ron_config.alt_network.alt_network_button_shadow_blur                                = v;
        ron_config.clock.alt_clock_button_shadow_blur                                        = v;
    }

    if let Some(v) = &ron_config.general_style.general_button_shadow_color
    {
        ron_config.disk.disk_button_shadow_color                                              = Some(*v);
        ron_config.tray.tray_button_shadow_color                                              = Some(*v);
        ron_config.focused_window.focused_window_button_shadow_color                          = Some(*v);
        ron_config.cpu.cpu_button_shadow_color                                                = Some(*v);
        ron_config.cpu_temp.cpu_temp_button_shadow_color                                      = Some(*v);
        ron_config.media_player_metadata.media_player_metadata_button_shadow_color            = Some(*v);
        ron_config.media_player_button.media_player_button_shadow_color                       = Some(*v);
        ron_config.network.network_button_shadow_color                                        = Some(*v);
        ron_config.ram.ram_button_shadow_color                                                = Some(*v);
        ron_config.volume_output.volume_output_button_shadow_color                            = Some(*v);
        ron_config.clock.clock_button_shadow_color                                            = Some(*v);
        ron_config.volume_input.volume_input_button_shadow_color                              = Some(*v);
        ron_config.workspace.workspace_button_shadow_color                                    = Some(*v);
    }

    if let Some(v) = ron_config.general_style.general_button_shadow_x
    {
        ron_config.disk.disk_button_shadow_x                                              = v;
        ron_config.tray.tray_button_shadow_x                                              = v;
        ron_config.focused_window.focused_window_button_shadow_x                          = v;
        ron_config.cpu.cpu_button_shadow_x                                                = v;
        ron_config.cpu_temp.cpu_temp_button_shadow_x                                      = v;
        ron_config.media_player_metadata.media_player_metadata_button_shadow_x            = v;
        ron_config.media_player_button.media_player_button_shadow_x                       = v;
        ron_config.network.network_button_shadow_x                                        = v;
        ron_config.ram.ram_button_shadow_x                                                = v;
        ron_config.volume_output.volume_output_button_shadow_x                            = v;
        ron_config.clock.clock_button_shadow_x                                            = v;
        ron_config.volume_input.volume_input_button_shadow_x                              = v;
        ron_config.workspace.workspace_button_shadow_x                                    = v;
    }

    if let Some(v) = ron_config.general_style.general_button_shadow_y
    {
        ron_config.disk.disk_button_shadow_y                                              = v;
        ron_config.tray.tray_button_shadow_y                                              = v;
        ron_config.focused_window.focused_window_button_shadow_y                          = v;
        ron_config.cpu.cpu_button_shadow_y                                                = v;
        ron_config.cpu_temp.cpu_temp_button_shadow_y                                      = v;
        ron_config.media_player_metadata.media_player_metadata_button_shadow_y            = v;
        ron_config.media_player_button.media_player_button_shadow_y                       = v;
        ron_config.network.network_button_shadow_y                                        = v;
        ron_config.ram.ram_button_shadow_y                                                = v;
        ron_config.volume_output.volume_output_button_shadow_y                            = v;
        ron_config.clock.clock_button_shadow_y                                            = v;
        ron_config.volume_input.volume_input_button_shadow_y                              = v;
        ron_config.workspace.workspace_button_shadow_y                                    = v;
    }

    if let Some(v) = ron_config.general_style.general_button_shadow_blur
    {
        ron_config.disk.disk_button_shadow_blur                                              = v;
        ron_config.tray.tray_button_shadow_blur                                              = v;
        ron_config.focused_window.focused_window_button_shadow_blur                          = v;
        ron_config.cpu.cpu_button_shadow_blur                                                = v;
        ron_config.cpu_temp.cpu_temp_button_shadow_blur                                      = v;
        ron_config.media_player_metadata.media_player_metadata_button_shadow_blur            = v;
        ron_config.media_player_button.media_player_button_shadow_blur                       = v;
        ron_config.network.network_button_shadow_blur                                        = v;
        ron_config.ram.ram_button_shadow_blur                                                = v;
        ron_config.volume_output.volume_output_button_shadow_blur                            = v;
        ron_config.clock.clock_button_shadow_blur                                            = v;
        ron_config.volume_input.volume_input_button_shadow_blur                              = v;
        ron_config.workspace.workspace_button_shadow_blur                                    = v;
    }
}
