export default function Spacer(node) {
    const size = (node.props && node.props.size) || 0;
    const el = document.createElement("div");
    if (size > 0) {
        el.style.width = size + "px";
        el.style.height = size + "px";
        el.style.flex = "0 0 auto";
    } else {
        el.style.flex = "1 1 auto";
    }
    return el;
}
