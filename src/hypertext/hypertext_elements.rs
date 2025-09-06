use hypertext::define_elements;

define_elements! {
    svg {
        id
        class
        style
        xmlns
        fill
        viewBox
        stroke_width
        stroke
        aria_hidden
        data_slot
        stroke_linecap
    }
    path {
        d
        stroke_linecap
        stroke_linejoin
    }
    rect {
        width
        height
        x
        y
        rx
    }
}
