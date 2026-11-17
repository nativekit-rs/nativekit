export default function Grid(node) {
    const cols = Math.max(1, (node.props && node.props.columns) || 2);
    const spacing = (node.props && node.props.spacing) || 12;
    const children = node.children || [];
    const hdrNode = children.find(c => c.name === "section_header");
    const ftrNode = children.find(c => c.name === "section_footer");
    const bodyChildren = children.filter(c => c.name !== "section_header" && c.name !== "section_footer").map(c => renderNode(c));
    return h("div", { className: "nk-grid-wrap", style: { display: "flex", flexDirection: "column", gap: "8px", width: "100%" } }, [
        hdrNode && hdrNode.children[0] ? h("div", { style: { padding: "0 4px" } }, renderNode(hdrNode.children[0])) : null,
        h("div", { className: "nk-grid", style: { display: "grid", gridTemplateColumns: `repeat(${cols}, minmax(0, 1fr))`, gap: `${spacing}px`, width: "100%" } }, bodyChildren),
        ftrNode && ftrNode.children[0] ? h("div", { style: { padding: "0 4px", opacity: "0.7", fontSize: "12px" } }, renderNode(ftrNode.children[0])) : null,
    ]);
}
