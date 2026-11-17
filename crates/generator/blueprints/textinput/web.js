export default function TextInput(node) {
    const { value = "", placeholder = "", on_change = "" } = node.props || {};
    const el = h("input", {
        className: "nk-textinput",
        attrs: { type: "text", placeholder: placeholder, value: value },
        style: {
            width: "100%",
            padding: "10px 12px",
            fontSize: "15px",
            borderRadius: "10px",
            border: "1px solid var(--rn-outline-variant, #d1d1d6)",
            backgroundColor: "var(--rn-surface, #ffffff)",
            color: "var(--rn-text, #000000)",
            outline: "none",
            boxSizing: "border-box",
        },
        onInput: (e) => {
            const newVal = e.target.value;
            if (el.__action && newVal !== el.__lastSent) {
                el.__lastSent = newVal;
                dispatch(el.__action, { value: newVal });
            }
        },
    });
    el.value = value;
    el.__lastSent = value;
    el.__action = on_change;
    return el;
}
