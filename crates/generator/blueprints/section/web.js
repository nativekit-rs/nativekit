export default function Section(node) {
    const children = node.children || [];
    const hdrNode = children.find(c => c.name === "section_header");
    const ftrNode = children.find(c => c.name === "section_footer");
    const bodyChildren = children.filter(c => c.name !== "section_header" && c.name !== "section_footer");
    const rows = bodyChildren.map((child, idx, arr) => {
        const isLast = idx === arr.length - 1;
        const isBtn = child.name === "button";
        const isPicker = Boolean(child.props && child.props.is_picker);
        const act = (child.props && child.props.action) || "";
        const col = (child.props && child.props.color) || (isBtn && !isPicker ? "primary" : "");
        const rowBg = (child.props && child.props.background) || "";
        const rowGrad = (child.props && child.props.background_gradient) || null;
        const hasRowBg = Boolean(rowBg || rowGrad);
        const cleanChild = hasRowBg
            ? { ...child, props: Object.fromEntries(Object.entries(child.props || {}).filter(([k]) => k !== "background" && k !== "background_gradient")) }
            : child;
        const rowContent = isPicker
            ? [
                h("span", { text: child.props.picker_title || "Select", style: { fontWeight: "400", color: "var(--rn-text)" } }),
                h("span", { text: (child.props.picker_value || "") + "  ›", style: { color: "var(--rn-on-surface-variant, #8e8e93)", fontSize: "15px" } })
              ]
            : (isBtn ? renderChildren(cleanChild) : renderNode(cleanChild));
        const rowEl = h("div", {
            className: "nk-section-row" + (isBtn ? " clickable" : ""),
            fgColor: col,
            style: {
                padding: "13px 16px",
                borderBottom: isLast ? "none" : "1px solid var(--rn-outline-variant, rgba(128,128,128,0.2))",
                display: "flex", alignItems: "center", justifyContent: isPicker ? "space-between" : "flex-start",
                gap: "8px", width: "100%", cursor: isBtn ? "pointer" : "default",
            },
            onClick: (isBtn && act) ? () => dispatch(act, null) : null,
        }, rowContent);
        if (rowGrad) applyWebGradient(rowEl, rowGrad);
        else if (rowBg) applyWebBgColor(rowEl, rowBg);
        return rowEl;
    });
    return h("div", { className: "nk-section", style: { display: "flex", flexDirection: "column", gap: "6px", width: "100%" } }, [
        hdrNode && hdrNode.children[0] ? h("div", { style: { padding: "0 8px" } }, renderNode(hdrNode.children[0])) : null,
        h("div", { style: { backgroundColor: "var(--rn-surface, #ffffff)", borderRadius: "14px", overflow: "hidden", border: "1px solid var(--rn-outline-variant, rgba(128,128,128,0.15))" } }, rows),
        ftrNode && ftrNode.children[0] ? h("div", { style: { padding: "0 12px", opacity: "0.7", fontSize: "12px" } }, renderNode(ftrNode.children[0])) : null,
    ]);
}
