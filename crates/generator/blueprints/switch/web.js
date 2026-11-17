export default function Switch(node) {
    const { on_change = "", value = false } = node.props || {};
    const el = h("input", {
        attrs: { type: "checkbox" },
        onChange: (e) => { if (on_change) dispatch(on_change, e.target.checked); },
    });
    el.checked = Boolean(value);
    return el;
}
