import SwiftUI

@ViewBuilder
func ImageView(node: DynamicUINode, dispatch: @escaping (String, [String: Any]?) -> Void) -> some View {
    let src = node.props["src"] as? String ?? ""
    let size = node.props["size"] as? Int ?? 0
    let w = node.props["width"] as? Int ?? size
    let h = node.props["height"] as? Int ?? (size > 0 ? size : 180)
    let radius = CGFloat(node.props["corner_radius"] as? Int ?? 0)
    let mode = node.props["content_mode"] as? String ?? "cover"
    let opacity = Double(node.props["opacity"] as? Int ?? 100) / 100.0
    let isContain = (mode == "contain" || mode == "fit")

    if src.hasPrefix("http://") || src.hasPrefix("https://"), let url = URL(string: src) {
        AsyncImage(url: url) { phase in
            switch phase {
            case .success(let image):
                if isContain { image.resizable().scaledToFit() }
                else { image.resizable().scaledToFill() }
            default:
                RoundedRectangle(cornerRadius: radius, style: .continuous)
                    .fill(Color.secondary.opacity(0.14))
                    .overlay(ProgressView())
            }
        }
        .frame(width: w > 0 ? CGFloat(w) : nil, height: CGFloat(h))
        .frame(maxWidth: w > 0 ? nil : .infinity)
        .clipped()
        .clipShape(RoundedRectangle(cornerRadius: radius, style: .continuous))
        .opacity(opacity)
    } else {
        let cleanName = ((src as NSString).lastPathComponent as NSString).deletingPathExtension
        if UIImage(named: cleanName) != nil {
            Group {
                if isContain { Image(cleanName).resizable().scaledToFit() }
                else { Image(cleanName).resizable().scaledToFill() }
            }
            .frame(width: w > 0 ? CGFloat(w) : nil, height: CGFloat(h))
            .frame(maxWidth: w > 0 ? nil : .infinity)
            .clipped()
            .clipShape(RoundedRectangle(cornerRadius: radius, style: .continuous))
            .opacity(opacity)
        } else if let devUrl = URL(string: "http://127.0.0.1:5824/" + src.replacingOccurrences(of: "./", with: "").trimmingCharacters(in: CharacterSet(charactersIn: "/"))) {
            AsyncImage(url: devUrl) { phase in
                if case .success(let image) = phase {
                    if isContain { image.resizable().scaledToFit() } else { image.resizable().scaledToFill() }
                } else {
                    RoundedRectangle(cornerRadius: radius, style: .continuous).fill(Color.secondary.opacity(0.14))
                }
            }
            .frame(width: w > 0 ? CGFloat(w) : nil, height: CGFloat(h))
            .frame(maxWidth: w > 0 ? nil : .infinity)
            .clipped()
            .clipShape(RoundedRectangle(cornerRadius: radius, style: .continuous))
            .opacity(opacity)
        }
    }
}
