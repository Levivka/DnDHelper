# [allow (non_snake_case)] # [allow (non_camel_case_types)] # [allow (clippy :: style)] # [allow (clippy :: complexity)] # [allow (unused_braces , unused_parens)] # [allow (clippy :: erasing_op)] # [allow (clippy :: approx_constant)] # [allow (clippy :: eq_op)] # [allow (clippy :: cmp_owned)] # [allow (clippy :: redundant_clone)] # [allow (clippy :: overly_complex_bool_expr)] mod slint_generatedMainWindow {
     use slint :: private_unstable_api :: re_exports as sp ;
     # [allow (unused_imports)] use sp :: {
         RepeatedItemTree as _ , ModelExt as _ , Model as _ , Float as _ }
     ;
     # [derive (Default , PartialEq , Debug , Clone)] pub struct r#TextStyle {
         pub r#font_size : f32 , pub r#font_weight : i32 }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerColorSchemeSelector_18 {
         r#dark_color_scheme : sp :: Property < bool > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , }
     impl InnerColorSchemeSelector_18 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             {
                 slint :: private_unstable_api :: set_property_binding ({
                     * & InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme }
                 . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_ref () ;
                     (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . dark_color_scheme ()) as _ }
                ) ;
                 }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerCosmicPalette_19 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , }
     impl InnerCosmicPalette_19 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin] struct InnerStyleMetrics_20 {
         root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , }
     impl InnerStyleMetrics_20 {
         fn new () -> :: core :: pin :: Pin < sp :: Rc < Self >> {
             sp :: Rc :: pin (Self :: default ()) }
         fn init (self : :: core :: pin :: Pin < sp :: Rc < Self >> , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) {
             # ! [allow (unused)] self . root . set (sp :: VRc :: downgrade (root)) ;
             let self_rc = self ;
             let _self = self_rc . as_ref () ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] pub struct InnerMainWindow {
         r#root_1 : sp :: r#WindowItem , r#text_3 : sp :: r#Text , r#_opacity_4 : sp :: r#Opacity , r#empty_5 : sp :: r#Empty , r#background_6 : sp :: r#BasicBorderRectangle , r#state_layer_12 : sp :: r#TouchArea , r#focus_scope_13 : sp :: r#FocusScope , r#overlay_15 : sp :: r#BasicBorderRectangle , r#root_1__opacity_4_dummy : sp :: Property < sp :: LogicalLength > , r#root_1_background_6_width : sp :: Property < sp :: LogicalLength > , r#root_1_background_6_x : sp :: Property < sp :: LogicalLength > , r#root_1_background_6_y : sp :: Property < sp :: LogicalLength > , r#root_1_base_14_enabled : sp :: Property < bool > , r#root_1_base_14_focus_boder_margin : sp :: Property < sp :: LogicalLength > , r#root_1_base_14_has_focus : sp :: Property < bool > , r#root_1_base_14_state : sp :: Property < i32 > , r#root_1_counter : sp :: Property < i32 > , r#root_1_empty_2_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_empty_2_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_2_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_empty_5_checkable : sp :: Property < bool > , r#root_1_empty_5_checked : sp :: Property < bool > , r#root_1_empty_5_height : sp :: Property < sp :: LogicalLength > , r#root_1_empty_5_icon : sp :: Property < sp :: Image > , r#root_1_empty_5_text_color : sp :: Property < slint :: Brush > , r#root_1_focus_scope_13_y : sp :: Property < sp :: LogicalLength > , r#root_1_layout_7_layout_cache : sp :: Property < sp :: SharedVector < sp :: Coord , > > , r#root_1_layout_7_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layout_7_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_h : sp :: Property < sp :: LayoutInfo > , r#root_1_layoutinfo_v : sp :: Property < sp :: LayoutInfo > , r#root_1_overlay_15_x : sp :: Property < sp :: LogicalLength > , r#root_1_overlay_15_y : sp :: Property < sp :: LogicalLength > , r#root_1_state_layer_12_x : sp :: Property < sp :: LogicalLength > , r#root_1_state_layer_12_y : sp :: Property < sp :: LogicalLength > , r#root_1_x : sp :: Property < sp :: LogicalLength > , r#root_1_y : sp :: Property < sp :: LogicalLength > , r#root_1_empty_5_clicked : sp :: Callback < () , () > , repeater0 : sp :: Repeater < InnerComponent_image_8 > , repeater1 : sp :: Repeater < InnerComponent_text_10 > , repeater2 : sp :: Repeater < InnerComponent_rectangle_16 > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerMainWindow >> , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , globals : Globals_MainWindow , window_adapter_ : sp :: OnceCell < sp :: WindowAdapterRc > , }
     impl InnerMainWindow {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             _self . repeater0 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new ({
                         let r#tmp_empty_5_icon = sp :: Image :: default () ;
                         ;
                         (((((r#tmp_empty_5_icon . clone () . size ()) . r#width as f64) > (0f64 as f64))) && ((((r#tmp_empty_5_icon . clone () . size ()) . r#height as f64) > (0f64 as f64)))) }
                     as bool)) as _ }
                 }
            ) ;
             _self . repeater1 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (true as bool)) as _ }
                 }
            ) ;
             _self . repeater2 . set_model_binding ({
                 let self_weak = sp :: VRcMapped :: downgrade (& self_rc) ;
                 move || {
                     let self_rc = self_weak . upgrade () . unwrap () ;
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: ModelRc :: new (((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_has_focus }
                    ) . apply_pin (_self) . get ()) && (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_enabled }
                    ) . apply_pin (_self) . get ())) as bool)) as _ }
                 }
            ) ;
             sp :: Property :: link_two_way (({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#state_layer_12 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#focus_scope_13 }
             + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self)) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4279966491f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4292335575f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_background_6_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_enabled }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#focus_scope_13 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_focus_boder_margin }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (3f64 as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_has_focus }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#focus_scope_13 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#has_focus) . apply_pin (_self) . get ()) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_state }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#state_layer_12 }
                     + sp :: r#TouchArea :: FIELD_OFFSETS . r#pressed) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (if ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#state_layer_12 }
                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#has_hover) . apply_pin (_self) . get () {
                             2f64 }
                         else {
                             (if ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_checked }
                            ) . apply_pin (_self) . get () {
                                 3f64 }
                             else {
                                 (0f64) as _ }
                            ) as _ }
                        ) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                         r#alignment : sp :: r#LayoutAlignment :: r#Stretch as _ , r#cells : sp :: Slice :: from_slice (& [{
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = sp :: Item :: layout_info (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
                            ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                             the_struct }
                         , {
                             let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                             the_struct . r#constraint = {
                                 let r#layout_info = (((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + (({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layoutinfo_v }
                                ) . apply_pin (_self) . get ()))))) + ((({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                ) + ({
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                     the_struct . r#max_percent = 100f64 as _ ;
                                     the_struct . r#min = 0f64 as _ ;
                                     the_struct . r#min_percent = 0f64 as _ ;
                                     the_struct . r#preferred = 0f64 as _ ;
                                     the_struct . r#stretch = 1f64 as _ ;
                                     the_struct }
                                )))) ;
                                 ;
                                 {
                                     let mut the_struct = sp :: LayoutInfo :: default () ;
                                     the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                     the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                     the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layoutinfo_v }
                                    ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                     the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                     the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                     the_struct . r#stretch = 0f64 as _ ;
                                     the_struct }
                                 }
                             as _ ;
                             the_struct }
                        ]) as _ , r#padding : {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 8f64 as _ ;
                             the_struct . r#end = 8f64 as _ ;
                             the_struct }
                         as _ , r#size : ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as _ , r#spacing : 8f64 as _ , }
                     as _ , sp :: Slice :: from_slice (& []) as _)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info_ortho (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
                        ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layoutinfo_h }
                            ) . apply_pin (_self) . get ()))))) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layoutinfo_h }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
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
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: r#box_layout_info (sp :: Slice :: from_slice (& [{
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = sp :: Item :: layout_info (({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
                        ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) as _ ;
                         the_struct }
                     , {
                         let mut the_struct = sp :: BoxLayoutCellData :: default () ;
                         the_struct . r#constraint = {
                             let r#layout_info = (((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + (({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layoutinfo_v }
                            ) . apply_pin (_self) . get ()))))) + ((({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            ) + ({
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = 340282346638528860000000000000000000000f64 as _ ;
                                 the_struct . r#max_percent = 100f64 as _ ;
                                 the_struct . r#min = 0f64 as _ ;
                                 the_struct . r#min_percent = 0f64 as _ ;
                                 the_struct . r#preferred = 0f64 as _ ;
                                 the_struct . r#stretch = 1f64 as _ ;
                                 the_struct }
                            )))) ;
                             ;
                             {
                                 let mut the_struct = sp :: LayoutInfo :: default () ;
                                 the_struct . r#max = (r#layout_info . clone ()) . r#max as _ ;
                                 the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                                 the_struct . r#min = (32f64 as sp :: Coord) . max (((({
                                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layoutinfo_v }
                                ) . apply_pin (_self) . get ()) . r#min as sp :: Coord)) as _ ;
                                 the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                                 the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                                 the_struct . r#stretch = 0f64 as _ ;
                                 the_struct }
                             }
                         as _ ;
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
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_checked }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (false) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_clicked }
                ) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_counter }
                            ) . apply_pin (_self) . set (((({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_counter }
                            ) . apply_pin (_self) . get () as f64) + (1f64 as f64)) as _) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [3usize] as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_text_color }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if false {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4279637526f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4294967295f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4291085508f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4282203453f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layout_cache }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut r#repeated_indices = [0u32 ;
                         2 * 2usize] ;
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_8 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         r#repeated_indices [0usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [0usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerMainWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_10 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         r#repeated_indices [1usize * 2] = items_vec . len () as u32 ;
                         r#repeated_indices [1usize * 2 + 1] = internal_vec . len () as u32 ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         let r#repeated_indices = sp :: Slice :: from_slice (& r#repeated_indices) ;
                         sp :: r#solve_box_layout (& sp :: BoxLayoutData {
                             r#alignment : sp :: r#LayoutAlignment :: r#Center as _ , r#cells : r#cells . clone () as _ , r#padding : {
                                 let mut the_struct = sp :: Padding :: default () ;
                                 the_struct . r#begin = 12f64 as _ ;
                                 the_struct . r#end = 12f64 as _ ;
                                 the_struct }
                             as _ , r#size : ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_background_6_width }
                            ) . apply_pin (_self) . get () . get () as _ , r#spacing : 4f64 as _ , }
                         as _ , r#repeated_indices . clone () as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_8 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         InnerMainWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_10 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Horizontal)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info (r#cells . clone () as _ , 4f64 as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 12f64 as _ ;
                             the_struct . r#end = 12f64 as _ ;
                             the_struct }
                         as _ , sp :: r#LayoutAlignment :: r#Center as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let mut items_vec = sp :: Vec :: with_capacity (0usize + _self . repeater0 . len () + _self . repeater1 . len ()) ;
                         InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_image_8 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater0 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         InnerMainWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| {
                             InnerComponent_text_10 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into () }
                        ) ;
                         let internal_vec = _self . repeater1 . instances_vec () ;
                         for sub_comp in & internal_vec {
                             items_vec . push (sub_comp . as_pin_ref () . box_layout_data (sp :: Orientation :: Vertical)) }
                         let r#cells = sp :: Slice :: from_slice (& items_vec) ;
                         sp :: r#box_layout_info_ortho (r#cells . clone () as _ , & {
                             let mut the_struct = sp :: Padding :: default () ;
                             the_struct . r#begin = 5f64 as _ ;
                             the_struct . r#end = 5f64 as _ ;
                             the_struct }
                         as _) }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_h }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: Item :: layout_info (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                    ) . apply_pin (_self) , sp :: Orientation :: Vertical , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()))) + (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layoutinfo_v }
                    ) . apply_pin (_self) . get ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Slint Window")) as sp :: SharedString }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4291085508f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4280887593f64 as u32)) as _ }
                    ) . color ())) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                    ) . apply_pin (_self) . get () [1usize] as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (((sp :: SharedString :: from ("Counter: ")) + (sp :: SharedString :: from (sp :: format ! ("{}" , ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_counter }
                    ) . apply_pin (_self) . get ()) . as_str ()) . as_str ()))) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let r#tmp_empty_2_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#_opacity_4 }
                 + sp :: r#Opacity :: FIELD_OFFSETS . r#opacity) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if ((if ! ({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#focus_scope_13 }
                     + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) . get () {
                         1f64 }
                     else {
                         (0f64) as _ }
                     as f64) == (1f64 as f64)) {
                         0.5f64 }
                     else {
                         (1f64) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#background_6 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if false {
                         slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4284731615f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4278211162f64 as u32)) as _ }
                        ) }
                     else {
                         (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                             sp :: Color :: from_argb_encoded (4280690214f64 as u32) }
                         else {
                             (sp :: Color :: from_argb_encoded (4291282887f64 as u32)) as _ }
                        )) as _ }
                    ) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#background_6 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#state_layer_12 }
                 + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             if ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_checkable }
                            ) . apply_pin (_self) . get () {
                                 {
                                     ({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_checked }
                                    ) . apply_pin (_self) . set (! ({
                                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_checked }
                                    ) . apply_pin (_self) . get () as _) }
                                 }
                             else {
                                 ({
                                     }
                                ) as _ }
                             ;
                             ({
                                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_clicked }
                            ) . apply_pin (_self) . call (& () . into ()) }
                        ) ;
                         }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#focus_scope_13 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#enabled) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (true) as _ }
                ) ;
                 }
             {
                 # [allow (unreachable_code , unused)] slint :: private_unstable_api :: set_callback_handler (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#focus_scope_13 }
                 + sp :: r#FocusScope :: FIELD_OFFSETS . r#key_pressed) . apply_pin (_self) , & self_rc , {
                     move | self_rc , args | {
                         let _self = self_rc . as_pin_ref () ;
                         ({
                             let r#returned_expression0 = {
                                 let r#return_check_merge0 = if ! (((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from (" ")))) || ((((args . 0 . clone ()) . r#text) == (sp :: SharedString :: from ("\n"))))) {
                                     (true , sp :: r#EventResult :: r#Reject ,) }
                                 else {
                                     ((false , {
                                         ({
                                             * & InnerMainWindow :: FIELD_OFFSETS . r#state_layer_12 }
                                         + sp :: r#TouchArea :: FIELD_OFFSETS . r#clicked) . apply_pin (_self) . call (& () . into ()) ;
                                         sp :: r#EventResult :: r#Accept }
                                     ,)) as _ }
                                 ;
                                 ;
                                 if (r#return_check_merge0 . clone ()) . 0 {
                                     (false , {
                                         sp :: r#EventResult :: r#Reject }
                                     ,) }
                                 else {
                                     ((false , (r#return_check_merge0 . clone ()) . 1 ,)) as _ }
                                 }
                             ;
                             ;
                             if (r#returned_expression0 . clone ()) . 0 {
                                 sp :: r#EventResult :: r#Reject }
                             else {
                                 ((r#returned_expression0 . clone ()) . 1) as _ }
                             }
                        ) as _ }
                     }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#overlay_15 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ({
                         let r#tmp_base_14_state = ({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_state }
                        ) . apply_pin (_self) . get () ;
                         ;
                         if ((r#tmp_base_14_state . clone () as f64) == (1f64 as f64)) {
                             slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                                 sp :: Color :: from_argb_encoded (2148931094f64 as u32) }
                             else {
                                 (sp :: Color :: from_argb_encoded (2159984318f64 as u32)) as _ }
                            ) }
                         else {
                             (if ((r#tmp_base_14_state . clone () as f64) == (2f64 as f64)) {
                                 slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (862151523f64 as u32)) }
                             else {
                                 (if ((r#tmp_base_14_state . clone () as f64) == (3f64 as f64)) {
                                     slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                                         sp :: Color :: from_argb_encoded (1296911693f64 as u32) }
                                     else {
                                         (sp :: Color :: from_argb_encoded (865638552f64 as u32)) as _ }
                                    ) }
                                 else {
                                     (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                                ) as _ }
                            ) as _ }
                         }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#overlay_15 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (16f64 as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1__opacity_4_dummy }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#always_on_top) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_background_6_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_background_6_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#default_font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_checkable }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_icon }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_focus_scope_13_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#icon) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#no_frame) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_overlay_15_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_overlay_15_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_state_layer_12_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_state_layer_12_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
             + sp :: r#WindowItem :: FIELD_OFFSETS . r#title) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_x }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_y }
            ) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
             + sp :: r#Text :: FIELD_OFFSETS . r#wrap) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#background_6 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#background_6 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#background_6 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#state_layer_12 }
             + sp :: r#TouchArea :: FIELD_OFFSETS . r#mouse_cursor) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#overlay_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#overlay_15 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
             }
         pub fn user_init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self >) {
             let _self = self_rc . as_pin_ref () ;
             {
                 }
             ;
             {
                 {
                     {
                         }
                     }
                 }
             ;
             }
         fn visit_dynamic_children (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_8 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater0 . visit (order , visitor) }
                 1u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_10 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater1 . visit (order , visitor) }
                 2u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_16 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     _self . repeater2 . visit (order , visitor) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn layout_info (self : :: core :: pin :: Pin < & Self > , orientation : sp :: Orientation) -> sp :: LayoutInfo {
             # ! [allow (unused)] let _self = self ;
             match orientation {
                 sp :: Orientation :: Horizontal => ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_h }
                ) . apply_pin (_self) . get () , sp :: Orientation :: Vertical => ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_layoutinfo_v }
                ) . apply_pin (_self) . get () , }
             }
         fn subtree_range (self : :: core :: pin :: Pin < & Self > , dyn_index : u32) -> sp :: IndexRange {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_8 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater0 . range ()) }
                 1u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_10 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater1 . range ()) }
                 2u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_16 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     sp :: IndexRange :: from (_self . repeater2 . range ()) }
                 _ => panic ! ("invalid dyn_index {}" , dyn_index) , }
             }
         fn subtree_component (self : :: core :: pin :: Pin < & Self > , dyn_index : u32 , subtree_index : usize , result : & mut sp :: ItemTreeWeak) {
             # ! [allow (unused)] let _self = self ;
             match dyn_index {
                 0u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater0 . apply_pin (_self) . ensure_updated (|| InnerComponent_image_8 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater0 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 1u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater1 . apply_pin (_self) . ensure_updated (|| InnerComponent_text_10 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater1 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
                 2u32 => {
                     InnerMainWindow :: FIELD_OFFSETS . repeater2 . apply_pin (_self) . ensure_updated (|| InnerComponent_rectangle_16 :: new (_self . self_weak . get () . unwrap () . clone ()) . unwrap () . into ()) ;
                     if let Some (instance) = _self . repeater2 . instance_at (subtree_index) {
                         * result = sp :: VRc :: downgrade (& sp :: VRc :: into_dyn (instance)) ;
                         }
                     }
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
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                 + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 1u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [1usize] as sp :: Coord , {
                     let r#tmp_empty_2_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [0usize] as sp :: Coord ,) , 2u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_2_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as sp :: Coord , 8f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [2usize] as sp :: Coord ,) , 3u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_2_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1__opacity_4_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1__opacity_4_dummy }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 4u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_2_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_background_6_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_background_6_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 5u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as sp :: Coord , {
                     let r#tmp_empty_2_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_state_layer_12_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_state_layer_12_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 8u32 => (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , 0f64 as sp :: Coord , 0f64 as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_focus_scope_13_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , 9u32 => (((1f64 as f64) * (({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache }
                ) . apply_pin (_self) . get () [3usize] as f64)) as sp :: Coord , ((1f64 as f64) * ({
                     let r#tmp_empty_2_padding = 8f64 ;
                     ;
                     ((((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                 as f64)) as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_overlay_15_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#root_1_overlay_15_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 1u32 => sp :: r#AccessibleRole :: r#Text , 3u32 => sp :: r#AccessibleRole :: r#Button , _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 (1u32 , sp :: AccessibleStringProperty :: r#Label) => ({
                     * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . get () , (3u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Increase") , _ => :: core :: default :: Default :: default () , }
             }
         # [allow (dead_code , unused)] pub fn r#fn_state_layer_12_focus (self : :: core :: pin :: Pin < & Self > ,) -> () {
             let _self = self ;
             let args = () ;
             (sp :: WindowInner :: from_pub ((& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ()) . window ()) . set_focus_item (& sp :: ItemRc :: new (sp :: VRcMapped :: origin (& _self . self_weak . get () . unwrap () . upgrade () . unwrap ()) , _self . tree_index_of_first_child . get () + 8u32 - 1))) ;
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_image_8 {
         r#image_8 : sp :: r#ImageItem , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_image_8 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_image_8 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             sp :: Property :: link_two_way (({
                 * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) , (InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_icon) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ())) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#colorize) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (if false {
                         (InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () }
                     else {
                         (slint :: Brush :: SolidColor (sp :: Color :: from_argb_encoded (0f64 as u32))) as _ }
                    ) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
                 + sp :: r#ImageItem :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [3usize] as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set ({
                 (sp :: r#ImageFit :: r#Contain) as sp :: r#ImageFit }
            ) ;
             ({
                 * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (20f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             ({
                 * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_fit) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#image_rendering) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#source) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
             + sp :: r#ImageItem :: FIELD_OFFSETS . r#width) . apply_pin (_self) . set_constant () ;
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
                 sp :: Orientation :: Horizontal => {
                     let r#layout_info = sp :: Item :: layout_info (({
                         * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
                    ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) ;
                     ;
                     {
                         let mut the_struct = sp :: LayoutInfo :: default () ;
                         the_struct . r#max = 20f64 as _ ;
                         the_struct . r#max_percent = (r#layout_info . clone ()) . r#max_percent as _ ;
                         the_struct . r#min = 20f64 as _ ;
                         the_struct . r#min_percent = (r#layout_info . clone ()) . r#min_percent as _ ;
                         the_struct . r#preferred = (r#layout_info . clone ()) . r#preferred as _ ;
                         the_struct . r#stretch = (r#layout_info . clone ()) . r#stretch as _ ;
                         the_struct }
                     }
                 , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
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
                 0u32 => ((((((InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [3usize] as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord , 20f64 as sp :: Coord , {
                     let cache = (InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [0usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_image_8 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_image_8 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_image_8 :: FIELD_OFFSETS . r#image_8 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_image_8) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_image_8 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_image_8 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_image_8 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_image_8 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 6u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
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
     impl sp :: RepeatedItemTree for InnerComponent_image_8 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_image_8 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_text_10 {
         r#text_10 : sp :: r#Text , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_text_10 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_text_10 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 let _self = self_rc . as_pin_ref () ;
                 slint :: private_unstable_api :: set_animated_property_binding (({
                     * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     ((InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_5_text_color) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get ()) as _ }
                 , {
                     let mut the_struct = sp :: PropertyAnimation :: default () ;
                     the_struct . r#delay = 0f64 as _ ;
                     the_struct . r#duration = 150f64 as _ ;
                     the_struct . r#easing = sp :: EasingCurve :: Linear as _ ;
                     the_struct . r#iteration_count = 1f64 as _ ;
                     the_struct }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#font_size) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((1.1535f64 as f64) * (({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                     + sp :: WindowItem :: FIELD_OFFSETS . default_font_size) . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set ({
                 (400f64) as i32 }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#height) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ((((((InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [3usize] as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextHorizontalAlignment :: r#Center) as sp :: r#TextHorizontalAlignment }
            ) ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set ({
                 (sp :: SharedString :: from ("Increase")) as sp :: SharedString }
            ) ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set ({
                 (sp :: r#TextVerticalAlignment :: r#Center) as sp :: r#TextVerticalAlignment }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
                 + sp :: r#Text :: FIELD_OFFSETS . r#width) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new ({
                         let cache = (InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                         * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                     as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_family) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_italic) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#font_weight) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#horizontal_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#letter_spacing) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#overflow) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#text) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
             + sp :: r#Text :: FIELD_OFFSETS . r#vertical_alignment) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
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
                     * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
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
                 0u32 => ((((((InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [3usize] as f64) - (5f64 as f64)) as f64) - (5f64 as f64)) as sp :: Coord , {
                     let cache = (InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [3usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , {
                     let cache = (InnerMainWindow :: FIELD_OFFSETS . r#root_1_layout_7_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () ;
                     * cache . get ((cache [2usize] as usize) + 0f64 as usize * 2) . unwrap_or (& (0 as sp :: Coord)) }
                 as sp :: Coord , 5f64 as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
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
                 (0u32 , sp :: AccessibleStringProperty :: r#Label) => sp :: SharedString :: from ("Increase") , _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_text_10 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_text_10 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_text_10 :: FIELD_OFFSETS . r#text_10 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_text_10) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_text_10 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_text_10 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_text_10 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_text_10 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 7u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
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
     impl sp :: RepeatedItemTree for InnerComponent_text_10 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_text_10 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     # [derive (sp :: FieldOffsets , Default)] # [const_field_offset (sp :: const_field_offset)] # [repr (C)] # [pin_drop] struct InnerComponent_rectangle_16 {
         r#rectangle_16 : sp :: r#BasicBorderRectangle , r#rectangle_16_height : sp :: Property < sp :: LogicalLength > , r#rectangle_16_width : sp :: Property < sp :: LogicalLength > , r#rectangle_16_x : sp :: Property < sp :: LogicalLength > , r#rectangle_16_y : sp :: Property < sp :: LogicalLength > , self_weak : sp :: OnceCell < sp :: VWeakMapped < sp :: ItemTreeVTable , InnerComponent_rectangle_16 >> , parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > , root : sp :: OnceCell < sp :: VWeak < sp :: ItemTreeVTable , InnerMainWindow >> , tree_index : :: core :: cell :: Cell < u32 > , tree_index_of_first_child : :: core :: cell :: Cell < u32 > , }
     impl InnerComponent_rectangle_16 {
         pub fn init (self_rc : sp :: VRcMapped < sp :: ItemTreeVTable , Self > , root : & sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > , tree_index : u32 , tree_index_of_first_child : u32) {
             # ! [allow (unused)] let _self = self_rc . as_pin_ref () ;
             _self . self_weak . set (sp :: VRcMapped :: downgrade (& self_rc)) ;
             _self . root . set (sp :: VRc :: downgrade (root)) ;
             _self . tree_index . set (tree_index) ;
             _self . tree_index_of_first_child . set (tree_index_of_first_child) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_color) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (slint :: Brush :: SolidColor (if InnerColorSchemeSelector_18 :: FIELD_OFFSETS . r#dark_color_scheme . apply_pin (_self . root . get () . unwrap () . upgrade () . unwrap () . globals . global_ColorSchemeSelector_18 . as_ref ()) . get () {
                         sp :: Color :: from_argb_encoded (4284731615f64 as u32) }
                     else {
                         (sp :: Color :: from_argb_encoded (4278211162f64 as u32)) as _ }
                    )) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
                 + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((({
                         * & InnerMainWindow :: FIELD_OFFSETS . r#overlay_15 }
                     + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_radius) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) + ((InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_focus_boder_margin) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set ({
                 (sp :: LogicalLength :: new (1f64 as sp :: Coord)) as sp :: LogicalLength }
            ) ;
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_height }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64 as f64) * ((InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [3usize] as f64)) as f64) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_focus_boder_margin) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) * (2f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_width }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((1f64 as f64) * ({
                         let r#tmp_empty_2_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as f64)) as f64) + ((((InnerMainWindow :: FIELD_OFFSETS . r#root_1_base_14_focus_boder_margin) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) * (2f64 as f64)) as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_x }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((1f64 as f64) * ({
                         let r#tmp_empty_2_padding = 8f64 ;
                         ;
                         ((((({
                             * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
                         + sp :: r#WindowItem :: FIELD_OFFSETS . r#width) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () . get () as f64) - (r#tmp_empty_2_padding . clone () as f64)) as f64) - (r#tmp_empty_2_padding . clone () as f64)) }
                     as f64)) as f64) - (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_width }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             {
                 slint :: private_unstable_api :: set_property_binding (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_y }
                ) . apply_pin (_self) , & self_rc , move | self_rc | {
                     let _self = self_rc . as_pin_ref () ;
                     (sp :: LogicalLength :: new (((((((1f64 as f64) * ((InnerMainWindow :: FIELD_OFFSETS . r#root_1_empty_2_layout_cache) . apply_pin (_self . parent . upgrade () . unwrap () . as_pin_ref ()) . get () [3usize] as f64)) as f64) - (({
                         * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_height }
                    ) . apply_pin (_self) . get () . get () as f64)) as f64) / (2f64 as f64)) as sp :: Coord)) as _ }
                ) ;
                 }
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#background) . apply_pin (_self) . set_constant () ;
             ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
             + sp :: r#BasicBorderRectangle :: FIELD_OFFSETS . r#border_width) . apply_pin (_self) . set_constant () ;
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
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
                ) . apply_pin (_self) , sp :: Orientation :: Horizontal , (& _self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl ())) , sp :: Orientation :: Vertical => sp :: Item :: layout_info (({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
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
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_height }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_width }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_x }
                ) . apply_pin (_self) . get () . get () as sp :: Coord , ({
                     * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16_y }
                ) . apply_pin (_self) . get () . get () as sp :: Coord ,) , _ => return :: core :: default :: Default :: default () }
             ;
             sp :: euclid :: rect (x , y , w , h) }
         fn accessible_role (self : :: core :: pin :: Pin < & Self > , index : u32) -> sp :: AccessibleRole {
             # ! [allow (unused)] let _self = self ;
             match index {
                 _ => sp :: AccessibleRole :: default () , }
             }
         fn accessible_string_property (self : :: core :: pin :: Pin < & Self > , index : u32 , what : sp :: AccessibleStringProperty ,) -> sp :: SharedString {
             # ! [allow (unused)] let _self = self ;
             match (index , what) {
                 _ => :: core :: default :: Default :: default () , }
             }
         }
     impl InnerComponent_rectangle_16 {
         pub fn new (parent : sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow >) -> core :: result :: Result < sp :: VRc < sp :: ItemTreeVTable , Self > , slint :: PlatformError > {
             # ! [allow (unused)] slint :: private_unstable_api :: ensure_backend () ? ;
             let mut _self = Self :: default () ;
             _self . parent = parent . clone () as sp :: VWeakMapped :: < sp :: ItemTreeVTable , InnerMainWindow > ;
             let self_rc = sp :: VRc :: new (_self) ;
             let self_dyn_rc = sp :: VRc :: into_dyn (self_rc . clone ()) ;
             sp :: register_item_tree (& self_dyn_rc , (* & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap ()) . maybe_window_adapter_impl ()) ;
             Self :: init (sp :: VRc :: map (self_rc . clone () , | x | x) , & parent . upgrade () . unwrap () . root . get () . unwrap () . upgrade () . unwrap () , 0 , 1) ;
             core :: result :: Result :: Ok (self_rc) }
         fn item_tree () -> & 'static [sp :: ItemTreeNode] {
             const ITEM_TREE : [sp :: ItemTreeNode ;
             1usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerComponent_rectangle_16 , sp :: ItemVTable , sp :: AllowPin > ;
             1usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerComponent_rectangle_16 :: FIELD_OFFSETS . r#rectangle_16 }
            )])) }
         # [allow (unused)] fn window_adapter_impl (& self) -> sp :: Rc < dyn sp :: WindowAdapter > {
             self . root . get () . unwrap () . upgrade () . unwrap () . window_adapter_impl () }
         # [allow (unused)] fn maybe_window_adapter_impl (& self) -> sp :: Option < sp :: Rc < dyn sp :: WindowAdapter >> {
             self . root . get () . and_then (| root_weak | root_weak . upgrade ()) . and_then (| root | root . maybe_window_adapter_impl ()) }
         }
     const _ : () = {
         use slint :: private_unstable_api :: re_exports :: * ;
         ItemTreeVTable_static ! (static VT for self :: InnerComponent_rectangle_16) ;
         }
     ;
     impl sp :: PinnedDrop for InnerComponent_rectangle_16 {
         fn drop (self : core :: pin :: Pin < & mut InnerComponent_rectangle_16 >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerComponent_rectangle_16 {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerComponent_rectangle_16 > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
             if let Some ((parent_component , parent_index)) = self . parent . clone () . upgrade () . map (| sc | (sp :: VRcMapped :: origin (& sc) , sc . tree_index_of_first_child . get ())) {
                 * _result = sp :: ItemRc :: new (parent_component , parent_index + 10u32 - 1) . downgrade () ;
                 }
             }
         fn embed_component (self : :: core :: pin :: Pin < & Self > , _parent_component : & sp :: ItemTreeWeak , _item_tree_index : u32) -> bool {
             todo ! ("Components written in Rust can not get embedded yet.") }
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
     impl sp :: RepeatedItemTree for InnerComponent_rectangle_16 {
         type Data = () ;
         fn update (& self , _index : usize , _data : Self :: Data) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             let _self = self_rc . as_pin_ref () ;
             }
         fn init (& self) {
             let self_rc = self . self_weak . get () . unwrap () . upgrade () . unwrap () ;
             InnerComponent_rectangle_16 :: user_init (sp :: VRcMapped :: map (self_rc , | x | x) ,) ;
             }
         fn box_layout_data (self : :: core :: pin :: Pin < & Self > , o : sp :: Orientation) -> sp :: BoxLayoutCellData {
             sp :: BoxLayoutCellData {
                 constraint : self . as_ref () . layout_info (o) }
             }
         }
     impl InnerMainWindow {
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
             11usize] = [sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 1u32 , parent_index : 0u32 , item_array_index : 0u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 0u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 1u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 1u32 , children_index : 3u32 , parent_index : 0u32 , item_array_index : 2u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : true , children_count : 2u32 , children_index : 4u32 , parent_index : 2u32 , item_array_index : 3u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 2u32 , children_index : 6u32 , parent_index : 3u32 , item_array_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 3u32 , children_index : 8u32 , parent_index : 3u32 , item_array_index : 5u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 0u32 , parent_index : 4u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 1u32 , parent_index : 4u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 11u32 , parent_index : 5u32 , item_array_index : 6u32 , }
             , sp :: ItemTreeNode :: Item {
                 is_accessible : false , children_count : 0u32 , children_index : 11u32 , parent_index : 5u32 , item_array_index : 7u32 , }
             , sp :: ItemTreeNode :: DynamicTree {
                 index : 2u32 , parent_index : 5u32 , }
            ] ;
             & ITEM_TREE }
         fn item_array () -> & 'static [sp :: VOffset < Self , sp :: ItemVTable , sp :: AllowPin >] {
             static ITEM_ARRAY : sp :: OnceBox < [sp :: VOffset < InnerMainWindow , sp :: ItemVTable , sp :: AllowPin > ;
             8usize] > = sp :: OnceBox :: new () ;
             & * ITEM_ARRAY . get_or_init (|| sp :: Box :: new ([sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#root_1 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#text_3 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#_opacity_4 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#empty_5 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#background_6 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#state_layer_12 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#focus_scope_13 }
            ) , sp :: VOffset :: new ({
                 * & InnerMainWindow :: FIELD_OFFSETS . r#overlay_15 }
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
         ItemTreeVTable_static ! (static VT for self :: InnerMainWindow) ;
         }
     ;
     impl sp :: PinnedDrop for InnerMainWindow {
         fn drop (self : core :: pin :: Pin < & mut InnerMainWindow >) {
             sp :: vtable :: new_vref ! (let vref : VRef < sp :: ItemTreeVTable > for sp :: ItemTree = self . as_ref () . get_ref ()) ;
             if let Some (wa) = self . maybe_window_adapter_impl () {
                 sp :: unregister_item_tree (self . as_ref () , vref , Self :: item_array () , & wa) ;
                 }
             }
         }
     impl sp :: ItemTree for InnerMainWindow {
         fn visit_children_item (self : :: core :: pin :: Pin < & Self > , index : isize , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ >) -> sp :: VisitChildrenResult {
             return sp :: visit_item_tree (self , & sp :: VRcMapped :: origin (& self . as_ref () . self_weak . get () . unwrap () . upgrade () . unwrap ()) , self . get_item_tree () . as_slice () , index , order , visitor , visit_dynamic) ;
             # [allow (unused)] fn visit_dynamic (_self : :: core :: pin :: Pin < & InnerMainWindow > , order : sp :: TraversalOrder , visitor : sp :: ItemVisitorRefMut < '_ > , dyn_index : u32) -> sp :: VisitChildrenResult {
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
     pub struct r#MainWindow (sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) ;
     impl r#MainWindow {
         pub fn new () -> core :: result :: Result < Self , slint :: PlatformError > {
             let inner = InnerMainWindow :: new () ? ;
             inner . globals . global_ColorSchemeSelector_18 . clone () . init (& inner) ;
             inner . globals . global_CosmicPalette_19 . clone () . init (& inner) ;
             inner . globals . global_StyleMetrics_20 . clone () . init (& inner) ;
             InnerMainWindow :: user_init (sp :: VRc :: map (inner . clone () , | x | x)) ;
             core :: result :: Result :: Ok (Self (inner)) }
         # [allow (dead_code)] fn get_counter (& self , _private_property : ()) {
             }
         # [allow (dead_code)] fn set_counter (& self , _private_property : ()) {
             }
         }
     impl From < r#MainWindow > for sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow > {
         fn from (value : r#MainWindow) -> Self {
             value . 0 }
         }
     impl slint :: ComponentHandle for r#MainWindow {
         type Inner = InnerMainWindow ;
         fn as_weak (& self) -> slint :: Weak < Self > {
             slint :: Weak :: new (& self . 0) }
         fn clone_strong (& self) -> Self {
             Self (self . 0 . clone ()) }
         fn from_inner (inner : sp :: VRc < sp :: ItemTreeVTable , InnerMainWindow >) -> Self {
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
     # [allow (dead_code)] struct Globals_MainWindow {
         global_ColorSchemeSelector_18 : :: core :: pin :: Pin < sp :: Rc < InnerColorSchemeSelector_18 >> , global_CosmicPalette_19 : :: core :: pin :: Pin < sp :: Rc < InnerCosmicPalette_19 >> , global_StyleMetrics_20 : :: core :: pin :: Pin < sp :: Rc < InnerStyleMetrics_20 >> , }
     impl :: core :: default :: Default for Globals_MainWindow {
         fn default () -> Self {
             Self {
                 global_ColorSchemeSelector_18 : InnerColorSchemeSelector_18 :: new () , global_CosmicPalette_19 : InnerCosmicPalette_19 :: new () , global_StyleMetrics_20 : InnerStyleMetrics_20 :: new () , }
             }
         }
     const _THE_SAME_VERSION_MUST_BE_USED_FOR_THE_COMPILER_AND_THE_RUNTIME : slint :: VersionCheck_1_5_1 = slint :: VersionCheck_1_5_1 ;
     }
 # [allow (unused_imports)] pub use slint_generatedMainWindow :: {
     r#MainWindow , r#TextStyle }
 ;
 # [allow (unused_imports)] pub use slint :: {
     ComponentHandle as _ , Global as _ , ModelExt as _ }
 ;
