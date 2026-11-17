export default function Image(node) {
    const { src = "", alt = "", width = 0, height = 0, size = 0, corner_radius = 0, content_mode = "cover", opacity = 100 } = node.props || {};
    const finalW = width || size;
    const finalH = height || size || 180;
    return h("img", {
        className: "nk-image",
        attrs: { src: src, alt: alt, loading: "lazy" },
        style: {
            width: finalW > 0 ? `${finalW}px` : "100%",
            height: `${finalH}px`,
            objectFit: (content_mode === "contain" || content_mode === "fit") ? "contain" : "cover",
            borderRadius: corner_radius > 0 ? `${corner_radius}px` : "0px",
            opacity: opacity < 100 ? String(opacity / 100.0) : "1",
            display: "block",
            flexShrink: "0",
        },
    });
}
