# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] # [allow (clippy :: overly_complex_bool_expr)] mod slint_generatedprofileWindow {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_35 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerprofileWindow >> , }
     impl InnerColorSchemeSelector_35 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_35 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerFluentPalette_36 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerprofileWindow >> , }
     impl InnerFluentPalette_36 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_37 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerprofileWindow >> , }
     impl InnerStyleMetrics_37 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct Innersidedown_root_12 {
         r#root_12 : sp :: r#BorderRectangle , r#buttons_13 : sp :: r#Empty , r#profile_14 : sp :: r#BasicBorderRectangle , r#touchArea_15 : sp :: r#TouchArea , r#text_16 : sp :: r#Text , r#image_17 : sp :: r#ImageItem , r#notes_18 : sp :: r#BasicBorderRectangle , r#touchArea_19 : sp :: r#TouchArea , r#text_20 : sp :: r#Text , r#image_21 : sp :: r#ImageItem , r#charlist_22 : sp :: r#BasicBorderRectangle , r#touchArea_23 : sp :: r#TouchArea , r#text_24 : sp :: r#Text , r#image_25 : sp :: r#ImageItem , r#dices_26 : sp :: r#BasicBorderRectangle , r#touchArea_27 : sp :: r#TouchArea , r#text_28 : sp :: r#Text , r#image_29 : sp :: r#ImageItem , r#settings_30 : sp :: r#BasicBorderRectangle , r#touchArea_31 : sp :: r#TouchArea , r#text_32 : sp :: r#Text , r#image_33 : sp :: r#ImageItem , r#root_12_buttons_13_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_12_buttons_13_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_buttons_13_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_buttons_13_x : sp :: Property < sp :: LogicalLength > , r#root_12_buttons_13_y : sp :: Property < sp :: LogicalLength > , r#root_12_charlist_22_height : sp :: Property < sp :: LogicalLength > , r#root_12_charlist_22_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_charlist_22_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_charlist_22_width : sp :: Property < sp :: LogicalLength > , r#root_12_charlist_22_y : sp :: Property < sp :: LogicalLength > , r#root_12_dices_26_height : sp :: Property < sp :: LogicalLength > , r#root_12_dices_26_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_dices_26_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_dices_26_width : sp :: Property < sp :: LogicalLength > , r#root_12_dices_26_y : sp :: Property < sp :: LogicalLength > , r#root_12_height : sp :: Property < sp :: LogicalLength > , r#root_12_image_17_horizontal_stretch : sp :: Property < f32 > , r#root_12_image_17_max_height : sp :: Property < sp :: LogicalLength > , r#root_12_image_17_max_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_17_min_height : sp :: Property < sp :: LogicalLength > , r#root_12_image_17_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_17_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_12_image_17_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_17_vertical_stretch : sp :: Property < f32 > , r#root_12_image_17_x : sp :: Property < sp :: LogicalLength > , r#root_12_image_17_y : sp :: Property < sp :: LogicalLength > , r#root_12_image_21_horizontal_stretch : sp :: Property < f32 > , r#root_12_image_21_max_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_21_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_21_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_21_x : sp :: Property < sp :: LogicalLength > , r#root_12_image_21_y : sp :: Property < sp :: LogicalLength > , r#root_12_image_25_horizontal_stretch : sp :: Property < f32 > , r#root_12_image_25_max_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_25_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_25_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_25_x : sp :: Property < sp :: LogicalLength > , r#root_12_image_25_y : sp :: Property < sp :: LogicalLength > , r#root_12_image_29_horizontal_stretch : sp :: Property < f32 > , r#root_12_image_29_max_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_29_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_29_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_29_x : sp :: Property < sp :: LogicalLength > , r#root_12_image_29_y : sp :: Property < sp :: LogicalLength > , r#root_12_image_33_horizontal_stretch : sp :: Property < f32 > , r#root_12_image_33_max_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_33_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_33_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_image_33_x : sp :: Property < sp :: LogicalLength > , r#root_12_image_33_y : sp :: Property < sp :: LogicalLength > , r#root_12_notes_18_height : sp :: Property < sp :: LogicalLength > , r#root_12_notes_18_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_notes_18_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_notes_18_width : sp :: Property < sp :: LogicalLength > , r#root_12_notes_18_y : sp :: Property < sp :: LogicalLength > , r#root_12_profile_14_height : sp :: Property < sp :: LogicalLength > , r#root_12_profile_14_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_profile_14_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_profile_14_width : sp :: Property < sp :: LogicalLength > , r#root_12_profile_14_y : sp :: Property < sp :: LogicalLength > , r#root_12_profile_selected : sp :: Property < bool > , r#root_12_settings_30_height : sp :: Property < sp :: LogicalLength > , r#root_12_settings_30_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_12_settings_30_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_12_settings_30_width : sp :: Property < sp :: LogicalLength > , r#root_12_settings_30_y : sp :: Property < sp :: LogicalLength > , r#root_12_text_16_min_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_16_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_16_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_16_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_16_x : sp :: Property < sp :: LogicalLength > , r#root_12_text_16_y : sp :: Property < sp :: LogicalLength > , r#root_12_text_20_min_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_20_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_20_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_20_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_20_x : sp :: Property < sp :: LogicalLength > , r#root_12_text_20_y : sp :: Property < sp :: LogicalLength > , r#root_12_text_24_min_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_24_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_24_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_24_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_24_x : sp :: Property < sp :: LogicalLength > , r#root_12_text_24_y : sp :: Property < sp :: LogicalLength > , r#root_12_text_28_min_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_28_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_28_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_28_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_28_x : sp :: Property < sp :: LogicalLength > , r#root_12_text_28_y : sp :: Property < sp :: LogicalLength > , r#root_12_text_32_min_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_32_min_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_32_preferred_height : sp :: Property < sp :: LogicalLength > , r#root_12_text_32_preferred_width : sp :: Property < sp :: LogicalLength > , r#root_12_text_32_x : sp :: Property < sp :: LogicalLength > , r#root_12_text_32_y : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_15_x : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_15_y : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_19_x : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_19_y : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_23_x : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_23_y : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_27_x : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_27_y : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_31_x : sp :: Property < sp :: LogicalLength > , r#root_12_touchArea_31_y : sp :: Property < sp :: LogicalLength > , r#root_12_width : sp :: Property < sp :: LogicalLength > , r#root_12_x : sp :: Property < sp :: LogicalLength > , r#root_12_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , Innersidedown_root_12 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerprofileWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl Innersidedown_root_12 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294020658f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_layoutinfo_h }
                                ) . apply_pin (_self) . get () ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ({
                                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                                    ) . apply_pin (_self) . get () . get () as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as _ , r#spacing : ((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.02f64 as f64)) as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_layoutinfo_h }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.02f64 as f64)) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Center as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((((({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                                ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((((({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                                ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = ({
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_layoutinfo_v }
                            ) . apply_pin (_self) . get () ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ({
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_height }
                                ) . apply_pin (_self) . get () . get () as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_selected }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) }
                     else {
                         (((((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (6f64 as f64)) }
                     else {
                         (((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (7f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_selected }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) }
                     else {
                         (((((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (6f64 as f64)) }
                     else {
                         (((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (7f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_max_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_vertical_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_height }
                    ) . apply_pin (_self) . get () . get () as f64) - ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_height }
                    ) . apply_pin (_self) . get () . get () as f64) - ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_height }
                    ) . apply_pin (_self) . get () . get () as f64) - ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_horizontal_stretch }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#stretch) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_max_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#max as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_height }
                    ) . apply_pin (_self) . get () . get () as f64) - ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_selected }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) }
                     else {
                         (((((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (6f64 as f64)) }
                     else {
                         (((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (7f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_max_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_min_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_preferred_height }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_vertical_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_selected }
                    ) . apply_pin (_self) . get () {
                         ((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (6f64 as f64)) }
                     else {
                         (((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (7f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_selected }
                    ) . apply_pin (_self) . get () {
                         ((((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) }
                     else {
                         (((((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                        ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + ({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_max_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_min_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_preferred_width }
                        ) . apply_pin (_self) . get () . get () as _ ;
                         the_struct . r#stretch = ({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_horizontal_stretch }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) + (sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (if false {
                         ((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (6f64 as f64)) }
                     else {
                         (((({
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                        ) . apply_pin (_self) . get () . get () as f64) / (7f64 as f64))) as _ }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_min_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_min_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#min as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_preferred_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_preferred_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((sp :: Item :: layout_info (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) . r#preferred as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_height }
                    ) . apply_pin (_self) . get () . get () as f64) - (({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                     + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#profile_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4291062553f64 as u32))) as slint :: Brush }
            ) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#profile_14 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_15 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#profile_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#profile_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_15 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_0 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#notes_18 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_19 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#notes_18 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#notes_18 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_19 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#charlist_22 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_23 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#charlist_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#charlist_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_23 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#dices_26 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_27 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#dices_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#dices_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_27 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#settings_30 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if ({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_31 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         sp :: Color :: from_argb_encoded (4294703092f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (16513012f64 as u32)) as _ }
                    )) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 100f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#settings_30 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (10f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#settings_30 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (2f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_preferred_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_min_height }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_preferred_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord) . max ((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_min_width }
                    ) . apply_pin (_self) . get () . get () as sp :: Coord)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((({
                         let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                         ;
                         (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                     as f64) * (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set ({
                 (sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg"))) as sp :: Image }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                    ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_15_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_15_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_19_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_19_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_23_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_23_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_27_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_27_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_31_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_31_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#profile_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#profile_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#profile_14 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_15 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_15 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#notes_18 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#notes_18 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#notes_18 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_19 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_19 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#charlist_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#charlist_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#charlist_22 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_23 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_23 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#dices_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#dices_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#dices_26 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_27 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_27 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#settings_30 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#settings_30 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#settings_30 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_31 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layoutinfo_h }
                ) . apply_pin (_self) . get ())) , sp :: Orientation :: Vertical => (({
                     let mut the_struct = sp :: LayoutInfo :: default () ;
                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                     the_struct . r#max_percent = 100f64 as _ ;
                     the_struct . r#min = 0f64 as _ ;
                     the_struct . r#min_percent = 0f64 as _ ;
                     the_struct . r#preferred = 0f64 as _ ;
                     the_struct . r#stretch = 1f64 as _ ;
                     the_struct }
                ) + (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layoutinfo_v }
                ) . apply_pin (_self) . get ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 2u32 => (((((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 3u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layout_cache }
                ) . apply_pin (_self) . get () [4usize] as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layout_cache }
                ) . apply_pin (_self) . get () [6usize] as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 6u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layout_cache }
                ) . apply_pin (_self) . get () [8usize] as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 7u32 => (((((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_15_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_15_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_16_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 9u32 => (((((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
                ) . apply_pin (_self) . get () . get () as f64) * (0.5f64 as f64)) as f64) * (1.3f64 as f64)) as sp :: Coord , ((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_17_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 10u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_19_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_19_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 11u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_20_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 12u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_1 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_21_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 13u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_23_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_23_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 14u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_24_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 15u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_2 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_25_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 16u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_27_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_27_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 17u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_28_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 18u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_3 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_29_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 19u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_31_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_31_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 20u32 => (({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_text_32_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 21u32 => ((({
                     let r#image_implicit_size = sp :: load_image_from_embedded_data (SLINT_EMBEDDED_RESOURCE_4 . into () , sp :: Slice :: from_slice (b"svg")) . size () ;
                     ;
                     (((r#image_implicit_size . clone ()) . r#height as f64) / ((r#image_implicit_size . clone ()) . r#width as f64)) }
                 as f64) * (((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as f64)) as sp :: Coord , ((({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_width }
                ) . apply_pin (_self) . get () . get () as f64) * (0.9f64 as f64)) as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_image_33_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 8u32 => sp :: r#AccessibleRole :: r#Text , 11u32 => sp :: r#AccessibleRole :: r#Text , 14u32 => sp :: r#AccessibleRole :: r#Text , 17u32 => sp :: r#AccessibleRole :: r#Text , 20u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (8u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (11u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (14u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (17u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , (20u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("") , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnertextBox_root_34 {
         r#root_34 : sp :: r#Text , r#root_34_x : sp :: Property < sp :: LogicalLength > , r#root_34_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnertextBox_root_34 >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerprofileWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnertextBox_root_34 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set ({
                 (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (4294703092f64 as u32))) as slint :: Brush }
            ) ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Metal Mania")) as sp :: SharedString }
            ) ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (14f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (10f64) as i32 }
            ) ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => sp :: Item :: layout_info (({
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 0u32 => sp :: r#AccessibleRole :: r#Text , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , _ => :: core :: default :: Default :: default () , }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerprofileWindow {
         r#root_1 : sp :: r#WindowItem , r#rectangle_2 : sp :: r#Rectangle , r#mainInfo_5 : sp :: r#Empty , r#profileInfo_6 : sp :: r#Empty , r#stats_9 : sp :: r#Empty , r#sidedown_4 : Innersidedown_root_12 , r#nickname_7 : InnertextBox_root_34 , r#id_8 : InnertextBox_root_34 , r#Characters_10 : InnertextBox_root_34 , r#adventures_11 : InnertextBox_root_34 , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_mainInfo_5_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_mainInfo_5_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_mainInfo_5_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_profileInfo_6_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_profileInfo_6_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_profileInfo_6_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_profileInfo_6_x : sp :: Property < sp :: LogicalLength > , r#root_1_rectangle_2_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_rectangle_2_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_sidedownMenu_3_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_sidedownMenu_3_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_sidedownMenu_3_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_stats_9_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_stats_9_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_stats_9_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_stats_9_x : sp :: Property < sp :: LogicalLength > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerprofileWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerprofileWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_profileWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerprofileWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             Innersidedown_root_12 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#sidedown_4 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 2u32 - 1 , tree_index_of_first_child + 4u32 - 1) ;
             InnertextBox_root_34 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#nickname_7 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 27u32 - 1 , tree_index_of_first_child + 29u32 - 1) ;
             InnertextBox_root_34 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#id_8 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 28u32 - 1 , tree_index_of_first_child + 29u32 - 1) ;
             InnertextBox_root_34 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#Characters_10 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 29u32 - 1 , tree_index_of_first_child + 31u32 - 1) ;
             InnertextBox_root_34 :: init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#adventures_11 }
             . apply_pin (x)) , & _self . root . get () . unwrap () . upgrade () . unwrap () , tree_index_of_first_child + 30u32 - 1 , tree_index_of_first_child + 31u32 - 1) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_35 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_35 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4280032284f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4294638330f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_rectangle_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_rectangle_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Start as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = ({
                                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layoutinfo_v }
                            ) . apply_pin (_self) . get () as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 0f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 20f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layoutinfo_h }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = ({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layoutinfo_v }
                        ) . apply_pin (_self) . get () as _ ;
                         the_struct }
                    ]) as _ , 20f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Start as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
                             + {
                                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
                             + {
                                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layout_cache }
                        ) . apply_pin (_self) . get () [1usize] as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
                         + {
                             * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
                         + {
                             * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
                         + {
                             * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
                         + {
                             * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 8f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_rectangle_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_3_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_rectangle_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((((({
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                         the_struct . r#max_percent = 100f64 as _ ;
                         the_struct . r#min = 0f64 as _ ;
                         the_struct . r#min_percent = 0f64 as _ ;
                         the_struct . r#preferred = 0f64 as _ ;
                         the_struct . r#stretch = 1f64 as _ ;
                         the_struct }
                    ) + (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_3_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) + (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_3_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#End as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
                                 + {
                                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layoutinfo_v }
                                ) . apply_pin (_self) . get ())) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = ((600f64 as f64) * (0.15f64 as f64)) as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = ((600f64 as f64) * (0.15f64 as f64)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 400f64 as _ ;
                             the_struct . r#end = 0f64 as _ ;
                             the_struct }
                         as _ , r#size : 600f64 as _ , r#spacing : 0f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_3_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = (({
                             let mut the_struct = sp :: LayoutInfo :: default () ;
                             the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                             the_struct . r#max_percent = 100f64 as _ ;
                             the_struct . r#min = 0f64 as _ ;
                             the_struct . r#min_percent = 0f64 as _ ;
                             the_struct . r#preferred = 0f64 as _ ;
                             the_struct . r#stretch = 1f64 as _ ;
                             the_struct }
                        ) + (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
                         + {
                             * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layoutinfo_h }
                        ) . apply_pin (_self) . get ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 0f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_3_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
                             + {
                                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_buttons_13_layoutinfo_v }
                            ) . apply_pin (_self) . get ())) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = ((600f64 as f64) * (0.15f64 as f64)) as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = ((600f64 as f64) * (0.15f64 as f64)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                                 the_struct }
                             }
                         as _ ;
                         the_struct }
                    ]) as _ , 0f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 400f64 as _ ;
                         the_struct . r#end = 0f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#End as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
                             + {
                                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
                             + {
                                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layout_cache }
                        ) . apply_pin (_self) . get () [3usize] as _ , r#spacing : 40f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
                         + {
                             * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
                         + {
                             * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
                         + {
                             * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
                         + {
                             * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                    ]) as _ , 40f64 as _ , & {
                         let mut the_struct = sp :: Padding :: default () ;
                         the_struct . r#begin = 8f64 as _ ;
                         the_struct . r#end = 8f64 as _ ;
                         the_struct }
                     as _ , sp :: r#LayoutAlignment :: r#Stretch as _)) as _ }
                ) ;
                 }
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#rectangle_2 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set ({
                 (slint :: Brush :: LinearGradient (sp :: LinearGradientBrush :: new (45f64 as _ , [sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4291456535f64 as u32) , position : 0f64 as _ }
                 , sp :: GradientStop {
                     color : sp :: Color :: from_argb_encoded (4294871583f64 as u32) , position : 1f64 as _ }
                ]))) as slint :: Brush }
            ) ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (((600f64 as f64) * (0.15f64 as f64)) as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_selected }
            ) . apply_pin (_self) . set ({
                 (true) as bool }
            ) ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (400f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
                 + {
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_3_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (32f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Levik")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_profileInfo_6_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_profileInfo_6_padding . clone () as f64)) as f64) - (r#tmp_profileInfo_6_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (18f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("12345")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_profileInfo_6_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_profileInfo_6_padding . clone () as f64)) as f64) - (r#tmp_profileInfo_6_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Characters:")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_stats_9_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_stats_9_padding . clone () as f64)) as f64) - (r#tmp_stats_9_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                    ) . apply_pin (_self) . get () [0usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (24f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Adventures:")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_stats_9_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_stats_9_padding . clone () as f64)) as f64) - (r#tmp_stats_9_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
            ) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (8f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                    ) . apply_pin (_self) . get () [2usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#rectangle_2 }
             + sp :: r#Rectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_bottom_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_bottom_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_left_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_top_right_radius) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
             + sp :: r#BorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_charlist_22_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_dices_26_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_height }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_notes_18_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_14_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_profile_selected }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_settings_30_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_15_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_15_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_19_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_19_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_23_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_23_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_27_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_27_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_31_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_touchArea_31_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_width }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34_x }
            ) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             Innersidedown_root_12 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#sidedown_4 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_34 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#nickname_7 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_34 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#id_8 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_34 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#Characters_10 }
             . apply_pin (x)) ,) ;
             InnertextBox_root_34 :: user_init (sp :: VRcMapped :: map (self_rc . clone () , | x | {
                 * & Self :: FIELD_OFFSETS . r#adventures_11 }
             . apply_pin (x)) ,) ;
             (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . renderer () . register_font_from_memory (SLINT_EMBEDDED_RESOURCE_5 . into ()) . unwrap () ;
             {
                 }
             ;
             {
                 }
             ;
             {
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             ;
             }
         fn index_property (self : :: core :: pin :: Pin < & Self >) -> usize {
             # ! [allow (unused)] let _self = self ;
             core :: usize :: MAX }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             # ! [allow (unused)] let _self = self ;
             let (h , w , x , y) = match index {
                 0u32 => (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (600f64 as sp :: Coord , 400f64 as sp :: Coord , ((((({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (400f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64) - (600f64 as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 2u32 => (((600f64 as f64) * (0.15f64 as f64)) as sp :: Coord , 400f64 as sp :: Coord , ({
                     InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
                 + {
                     * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_sidedownMenu_3_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ((((400f64 as f64) - (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord , ((((600f64 as f64) - (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord ,) , 25u32 => (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 26u32 => (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_mainInfo_5_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 27u32 => (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_profileInfo_6_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_profileInfo_6_padding . clone () as f64)) as f64) - (r#tmp_profileInfo_6_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 28u32 => (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_profileInfo_6_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_profileInfo_6_padding . clone () as f64)) as f64) - (r#tmp_profileInfo_6_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_profileInfo_6_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 29u32 => (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_stats_9_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_stats_9_padding . clone () as f64)) as f64) - (r#tmp_stats_9_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 30u32 => (({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_stats_9_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_stats_9_padding . clone () as f64)) as f64) - (r#tmp_stats_9_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1_stats_9_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 4u32 ..= 24u32 => return {
                     * & Self :: FIELD_OFFSETS . r#sidedown_4 }
                 . apply_pin (_self) . item_geometry (index - 4u32 + 1) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 27u32 => sp :: r#AccessibleRole :: r#Text , 28u32 => sp :: r#AccessibleRole :: r#Text , 29u32 => sp :: r#AccessibleRole :: r#Text , 30u32 => sp :: r#AccessibleRole :: r#Text , 2u32 => {
                     * & Self :: FIELD_OFFSETS . r#sidedown_4 }
                 . apply_pin (_self) . accessible_role (0) , 4u32 ..= 24u32 => {
                     * & Self :: FIELD_OFFSETS . r#sidedown_4 }
                 . apply_pin (_self) . accessible_role (index - 4u32 + 1) , 27u32 => {
                     * & Self :: FIELD_OFFSETS . r#nickname_7 }
                 . apply_pin (_self) . accessible_role (0) , 28u32 => {
                     * & Self :: FIELD_OFFSETS . r#id_8 }
                 . apply_pin (_self) . accessible_role (0) , 29u32 => {
                     * & Self :: FIELD_OFFSETS . r#Characters_10 }
                 . apply_pin (_self) . accessible_role (0) , 30u32 => {
                     * & Self :: FIELD_OFFSETS . r#adventures_11 }
                 . apply_pin (_self) . accessible_role (0) , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (27u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (28u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (29u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (30u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
                 + {
                     * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (2u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#sidedown_4 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (4u32 ..= 24u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#sidedown_4 }
                 . apply_pin (_self) . accessible_string_property (index - 4u32 + 1 , what) , (27u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#nickname_7 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (28u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#id_8 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (29u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#Characters_10 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , (30u32 , _) => {
                     * & Self :: FIELD_OFFSETS . r#adventures_11 }
                 . apply_pin (_self) . accessible_string_property (0 , what) , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerprofileWindow {
         pub fn new () -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & self_rc) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & self_rc , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             31usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 2u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 4u32 , parent_index : 1u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 25u32 , parent_index : 1u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 5u32 , children_index : 5u32 , parent_index : 2u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 10u32 , parent_index : 4u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 13u32 , parent_index : 4u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 16u32 , parent_index : 4u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 19u32 , parent_index : 4u32 , item_array_index : 8u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 22u32 , parent_index : 4u32 , item_array_index : 9u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 13u32 , parent_index : 5u32 , item_array_index : 10u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 13u32 , parent_index : 5u32 , item_array_index : 11u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 13u32 , parent_index : 5u32 , item_array_index : 12u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 16u32 , parent_index : 6u32 , item_array_index : 13u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 16u32 , parent_index : 6u32 , item_array_index : 14u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 16u32 , parent_index : 6u32 , item_array_index : 15u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 19u32 , parent_index : 7u32 , item_array_index : 16u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 19u32 , parent_index : 7u32 , item_array_index : 17u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 19u32 , parent_index : 7u32 , item_array_index : 18u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 22u32 , parent_index : 8u32 , item_array_index : 19u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 22u32 , parent_index : 8u32 , item_array_index : 20u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 22u32 , parent_index : 8u32 , item_array_index : 21u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 25u32 , parent_index : 9u32 , item_array_index : 22u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 25u32 , parent_index : 9u32 , item_array_index : 23u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 25u32 , parent_index : 9u32 , item_array_index : 24u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 27u32 , parent_index : 3u32 , item_array_index : 25u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 29u32 , parent_index : 3u32 , item_array_index : 26u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 29u32 , parent_index : 25u32 , item_array_index : 27u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 29u32 , parent_index : 25u32 , item_array_index : 28u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 31u32 , parent_index : 26u32 , item_array_index : 29u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 31u32 , parent_index : 26u32 , item_array_index : 30u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerprofileWindow , sp :: ItemVTable , sp :: AllowPin > ;
             31usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#rectangle_2 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#root_12 }
            ) , sp :: VOffset :: new ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#mainInfo_5 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#buttons_13 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#profile_14 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#notes_18 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#charlist_22 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#dices_26 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#settings_30 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_15 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_16 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_17 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_19 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_20 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_21 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_23 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_24 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_25 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_27 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_28 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_29 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#touchArea_31 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#text_32 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#sidedown_4 }
             + {
                 * & Innersidedown_root_12 :: FIELD_OFFSETS . r#image_33 }
            ) , sp :: VOffset :: new ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#profileInfo_6 }
            ) , sp :: VOffset :: new ({
                 * & InnerprofileWindow :: FIELD_OFFSETS . r#stats_9 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#nickname_7 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#id_8 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#Characters_10 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
            ) , sp :: VOffset :: new ({
                 InnerprofileWindow :: FIELD_OFFSETS . r#adventures_11 }
             + {
                 * & InnertextBox_root_34 :: FIELD_OFFSETS . r#root_34 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             sp :: Rc :: clone (self . window_adapter_ref () . unwrap ()) }
         fn window_adapter_ref (& self ,) -> sp :: Result < & sp :: Rc < dyn sp :: WindowAdapter > , slint :: PlatformError > {
             self . window_adapter_ . get_or_try_init (|| {
                 let adapter = slint :: private_unstable_api :: create_window_adapter () ? ;
                 let self_rc = sp :: VRcMapped :: origin (& self . self_weak . get () . unwrap () . upgrade () . unwrap () ,) ;
                 sp :: WindowInner :: from_pub (adapter . window ()) . set_component (& self_rc) ;
                 core :: result :: Result :: Ok (adapter) }
            ) }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . window_adapter_ . get () . cloned () }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerprofileWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerprofileWindow {
         fn drop (self : core :: pin :: Pin < & mut InnerprofileWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerprofileWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerprofileWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
                 _self . visit_dynamic_children (dyn_index , order , visitor) }
             }
         fn get_item_ref (self : :: core :: pin :: Pin < & Self > , index : u32) -> :: core :: pin :: Pin < sp :: ItemRef < '_ >> {
             match & self . get_item_tree () . as_slice () [index as usize] {
                 sp :: ItemTreeNode :: Item {
                     item_array_index , .. }
                 => {
                     Self :: item_array () [* item_array_index as usize] . apply_pin (self) }
                 sp :: ItemTreeNode :: DynamicTree {
                     .. }
                 => panic ! ("get_item_ref called on dynamic tree") , }
             }
         fn get_item_tree (self : :: core :: pin :: Pin < & Self >) -> sp :: Slice < '_ , sp :: ItemTreeNode > {
             Self :: item_tree () . into () }
         fn get_subtree_range (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: IndexRange {
             self . subtree_range (index) }
         fn get_subtree (self : :: core :: pin :: Pin < & Self > , index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             self . subtree_component (index , subtree_index , result) ;
             }
         fn subtree_index (self : :: core :: pin :: Pin < & Self >) -> usize {
             self . index_property () }
         fn parent_node (self : :: core :: pin :: Pin < & Self > , _result : & mut sp :: ItemWeak) {
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             false }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             self . layout_info (orientation) }
         fn item_geometry (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: LogicalRect {
             self . item_geometry (index) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             self . accessible_role (index) }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty , result : & mut sp :: SharedString ,) {
             * result = self . accessible_string_property (index , what) ;
             }
         fn window_adapter (self : :: core :: pin :: Pin < & Self > , do_create : bool , result : & mut sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> ,) {
             if do_create {
                 * result = sp :: Some (self . window_adapter_impl ()) ;
                 }
             else {
                 * result = self . maybe_window_adapter_impl () ;
                 }
             }
         }
     pub struct r#profileWindow (sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow >) ;
     impl r#profileWindow {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerprofileWindow :: new () ? ;
             inner . globals . global_ColorSchemeSelector_35 . clone () . init (& inner) ;
             inner . globals . global_FluentPalette_36 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_37 . clone () . init (& inner) ;
             InnerprofileWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         }
     impl From < r#profileWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow > {
         fn from (value : r#profileWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#profileWindow {
         type Inner = InnerprofileWindow ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : sp :: VRc < sp :: ItemTreeVTable , InnerprofileWindow >) -> Self {
             Self (inner) }
         fn run (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . show () ? ;
             slint :: run_event_loop () ? ;
             self . hide () ? ;
             core :: result :: Result :: Ok (()) }
         fn show (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . show () }
         fn hide (& self) -> core :: result :: Result < () , slint :: PlatformError > {
             self . 0 . window_adapter_ref () ? . window () . hide () }
         fn window (& self) -> & slint :: Window {
             self . 0 . window_adapter_ref () . unwrap () . window () }
         fn global < 'a , T : slint :: Global < 'a , Self >> (& 'a self) -> T {
             T :: get (& self) }
         }
     # [allow (dead_code)] struct Globals_profileWindow {
         global_ColorSchemeSelector_35 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_35 >> , global_FluentPalette_36 : :: core :: pin :: Pin < sp :: Rc < InnerFluentPalette_36 >> , global_StyleMetrics_37 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_37 >> , }
     impl :: core :: default :: Default for Globals_profileWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_35 : InnerColorSchemeSelector_35 :: new () , global_FluentPalette_36 : InnerFluentPalette_36 :: new () , global_StyleMetrics_37 : InnerStyleMetrics_37 :: new () , }
             }
         }
     static SLINT_EMBEDDED_RESOURCE_4 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/setting-2.svg") ;
     static SLINT_EMBEDDED_RESOURCE_5 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/fonts/MetalMania-Regular.ttf") ;
     static SLINT_EMBEDDED_RESOURCE_3 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/dices.svg") ;
     static SLINT_EMBEDDED_RESOURCE_2 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/charlist.svg") ;
     static SLINT_EMBEDDED_RESOURCE_1 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/book.svg") ;
     static SLINT_EMBEDDED_RESOURCE_0 : & 'static [u8] = :: core :: include_bytes ! ("/Users/levik/Desktop/rust/DnDHelper/ui/resources/user-edit.svg") ;
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_5_1 = slint :: VersionCheck_1_5_1 ;
     }
 # [allow (unused_imports)] pub use slint_generatedprofileWindow :: {
     r#profileWindow }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
