using System;
using System.Collections.Generic;
using System.IO;
using System.Net.Http;
using System.Runtime.InteropServices;
using System.Text;
using System.Text.Json;
using Microsoft.UI.Xaml;
using Microsoft.UI.Xaml.Controls;

namespace CargoNativeApp {
    public class DynamicUINode {
        public string Name { get; set; }
        public Dictionary<string, object> Props { get; set; }
        public List<DynamicUINode> Children { get; set; }
        public bool InList { get; set; }

        public DynamicUINode(string name, Dictionary<string, object> props, List<DynamicUINode> children, bool inList = false) {
            Name = name;
            Props = props;
            Children = children;
            InList = inList;
        }

        public static DynamicUINode Decode(BinaryReader reader, bool inList = false) {
            if (reader.BaseStream.Position >= reader.BaseStream.Length) return new DynamicUINode("error", new Dictionary<string, object>(), new List<DynamicUINode>(), inList);
            
            byte type = reader.ReadByte();
            if (type == 1) {
                string name = ReadStr(reader).ToLower();
                int propsCount = ReadU16(reader);
                var props = new Dictionary<string, object>();
                for (int i = 0; i < propsCount; i++) {
                    string k = ReadStr(reader);
                    byte vType = reader.ReadByte();
                    if (vType == 0) props[k] = ReadStr(reader);
                    else if (vType == 1) props[k] = reader.ReadByte() != 0;
                    else if (vType == 2) props[k] = (int)ReadU32(reader);
                    else if (vType == 3) props[k] = ReadStr(reader);
                    else if (vType == 4) props[k] = ReadStr(reader);
                    else if (vType == 5) {
                        int len = (int)ReadU32(reader);
                        byte[] b = reader.ReadBytes(len);
                        try {
                            var je = JsonSerializer.Deserialize<JsonElement>(Encoding.UTF8.GetString(b));
                            if (je.ValueKind == JsonValueKind.Object) {
                                props[k] = JsonSerializer.Deserialize<Dictionary<string, object>>(Encoding.UTF8.GetString(b));
                            } else {
                                props[k] = je;
                            }
                        } catch { }
                    }
                }
                int childrenCount = ReadU16(reader);
                var children = new List<DynamicUINode>();
                bool isNextList = name == "list";
                for (int i = 0; i < childrenCount; i++) {
                    children.Add(Decode(reader, isNextList));
                }
                return new DynamicUINode(name, props, children, inList);
            } else {
                int len = (int)ReadU32(reader);
                reader.BaseStream.Seek(len, SeekOrigin.Current);
                return new DynamicUINode("raw", new Dictionary<string, object>(), new List<DynamicUINode>(), inList);
            }
        }

        public DynamicUINode ApplyPatch(List<int> path, int pathIdx, byte pType, BinaryReader reader) {
            if (pathIdx == path.Count) {
                if (pType == 0) {
                    return Decode(reader, InList);
                } else if (pType == 1) {
                    int propCount = ReadU16(reader);
                    var newProps = new Dictionary<string, object>(Props);
                    for (int i = 0; i < propCount; i++) {
                        string k = ReadStr(reader);
                        byte vType = reader.ReadByte();
                        if (vType == 0) newProps[k] = ReadStr(reader);
                        else if (vType == 1) newProps[k] = reader.ReadByte() != 0;
                        else if (vType == 2) newProps[k] = (int)ReadU32(reader);
                        else if (vType == 3) newProps[k] = ReadStr(reader);
                        else if (vType == 4) newProps[k] = ReadStr(reader);
                        else if (vType == 5) {
                            int len = (int)ReadU32(reader);
                            byte[] b = reader.ReadBytes(len);
                            try {
                                var je = JsonSerializer.Deserialize<JsonElement>(Encoding.UTF8.GetString(b));
                            if (je.ValueKind == JsonValueKind.Object) {
                                newProps[k] = JsonSerializer.Deserialize<Dictionary<string, object>>(Encoding.UTF8.GetString(b));
                            } else {
                                newProps[k] = je;
                            }
                            } catch { }
                        }
                    }
                    return new DynamicUINode(Name, newProps, Children, InList);
                } else if (pType == 2) {
                    var newChildren = new List<DynamicUINode>(Children);
                    newChildren.Add(Decode(reader, Name == "list"));
                    return new DynamicUINode(Name, Props, newChildren, InList);
                } else if (pType == 3) {
                    int len = ReadU16(reader);
                    var newChildren = new List<DynamicUINode>(Children);
                    if (len < newChildren.Count) {
                        newChildren.RemoveRange(len, newChildren.Count - len);
                    }
                    return new DynamicUINode(Name, Props, newChildren, InList);
                }
                return this;
            }
            int childIdx = path[pathIdx];
            var updatedChildren = new List<DynamicUINode>(Children);
            if (childIdx < updatedChildren.Count) {
                updatedChildren[childIdx] = updatedChildren[childIdx].ApplyPatch(path, pathIdx + 1, pType, reader);
            }
            return new DynamicUINode(Name, Props, updatedChildren, InList);
        }

        public static string ReadStr(BinaryReader r) {
            int len = ReadU16(r);
            byte[] b = r.ReadBytes(len);
            return Encoding.UTF8.GetString(b);
        }
        public static int ReadU16(BinaryReader r) {
            byte[] b = r.ReadBytes(2);
            return b[0] | (b[1] << 8);
        }
        public static uint ReadU32(BinaryReader r) {
            byte[] b = r.ReadBytes(4);
            return (uint)(b[0] | (b[1] << 8) | (b[2] << 16) | (b[3] << 24));
        }
    }

    public partial class MainWindow : Window {
        [DllImport("engine.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr init_app();
        
        [DllImport("engine.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr get_ui(IntPtr app);
        
        [DllImport("engine.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern IntPtr dispatch_event(IntPtr app, [MarshalAs(UnmanagedType.LPStr)] string action, [MarshalAs(UnmanagedType.LPStr)] string payload);
        
        [UnmanagedFunctionPointer(CallingConvention.Cdecl)]
        private delegate void RefreshCallback();
        
        [DllImport("engine.dll", CallingConvention = CallingConvention.Cdecl)]
        private static extern void register_refresh_callback(RefreshCallback cb);

        private IntPtr appPtr;
        private DynamicUINode liveUiNode;
        private bool isDevMode = false;
        private DispatcherTimer timer;
        private HttpClient client = new HttpClient() { Timeout = TimeSpan.FromSeconds(2) };
        private string[] serverUrls;
        private string activeServerUrl = null;
        private RefreshCallback refreshCb;
        private string currentBackAction = "";
        private string runtimePrimaryHex = "";
        private string runtimeAccentMode = "app";

        public MainWindow() {
            InitializeComponent();

            this.Title = "My Native Kit App";

            try {
                ExtendsContentIntoTitleBar = true;
                SetTitleBar(AppTitleBar);
            } catch { }

            try {
                var hWnd = WinRT.Interop.WindowNative.GetWindowHandle(this);
                var windowId = Microsoft.UI.Win32Interop.GetWindowIdFromWindow(hWnd);
                var appWindow = Microsoft.UI.Windowing.AppWindow.GetFromWindowId(windowId);
                appWindow.Title = "My Native Kit App";

                var iconPath = System.IO.Path.Combine(AppContext.BaseDirectory, "app.ico");
                if (!System.IO.File.Exists(iconPath)) {
                    if (System.IO.File.Exists("app.ico")) iconPath = System.IO.Path.GetFullPath("app.ico");
                }
                if (System.IO.File.Exists(iconPath)) {
                    appWindow.SetIcon(iconPath);
                }
            } catch { }

            try {
                var logoPath = System.IO.Path.Combine(AppContext.BaseDirectory, "logo.png");
                if (!System.IO.File.Exists(logoPath)) {
                    if (System.IO.File.Exists("logo.png")) logoPath = System.IO.Path.GetFullPath("logo.png");
                }
                if (System.IO.File.Exists(logoPath)) {
                    var bitmap = new Microsoft.UI.Xaml.Media.Imaging.BitmapImage();
                    bitmap.UriSource = new Uri(logoPath, UriKind.Absolute);
                    AppLogoImage.Source = bitmap;
                }
            } catch { }

            WindowBackButton.Click += (s, e) => {
                if (!string.IsNullOrEmpty(currentBackAction)) {
                    Dispatch(currentBackAction, "");
                }
            };
            
            refreshCb = new RefreshCallback(() => {
                DispatcherQueue.TryEnqueue(() => {
                    RenderUINative();
                });
            });

            /* BEGIN_DEV_URL */
serverUrls = new string[] { "http://192.168.1.69:5824", "http://10.211.55.2:5824", "http://192.168.64.1:5824", "http://10.0.2.2:5824", "http://127.0.0.1:5824",  };
/* END_DEV_URL */
            
            try {
                if (isDevMode) throw new Exception("Forced dev mode");
                appPtr = init_app();
                register_refresh_callback(refreshCb);
                RenderUINative();
            } catch (DllNotFoundException) {
                isDevMode = true;
                timer = new DispatcherTimer();
                timer.Interval = TimeSpan.FromMilliseconds(500);
                timer.Tick += (s, e) => CheckReloadDev();
                timer.Start();
                FetchUIDev();
            } catch (Exception) {
                isDevMode = true;
                timer = new DispatcherTimer();
                timer.Interval = TimeSpan.FromMilliseconds(500);
                timer.Tick += (s, e) => CheckReloadDev();
                timer.Start();
                FetchUIDev();
            }
        }

        private void RenderUINative() {
            if (appPtr == IntPtr.Zero) return;
            IntPtr uiPtr = get_ui(appPtr);
            string b64 = Marshal.PtrToStringUTF8(uiPtr);
            if (string.IsNullOrEmpty(b64)) return;
            ProcessBase64(b64);
        }

        private ulong lastReloadTs = 0;
        
        private async void CheckReloadDev() {
            if (activeServerUrl == null) return;
            try {
                var response = await client.GetStringAsync(activeServerUrl + "/api/live_reload");
                if (!string.IsNullOrEmpty(response)) {
                    var json = JsonSerializer.Deserialize<Dictionary<string, object>>(response);
                    if (json != null && json.ContainsKey("timestamp")) {
                        ulong ts = ulong.Parse(json["timestamp"].ToString());
                        if (ts != lastReloadTs) {
                            lastReloadTs = ts;
                            FetchUIDev();
                        }
                    }
                }
            } catch { }
        }

        private async void FetchUIDev() {
            if (serverUrls == null) return;
            if (activeServerUrl == null) {
                foreach (var url in serverUrls) {
                    try {
                        var response = await client.GetStringAsync(url + "/api/ui_layout");
                        if (!string.IsNullOrEmpty(response)) {
                            activeServerUrl = url;
                            ProcessBase64(response);
                            return;
                        }
                    } catch { }
                }
            } else {
                try {
                    var response = await client.GetStringAsync(activeServerUrl + "/api/ui_layout");
                    if (string.IsNullOrEmpty(response)) return;
                    ProcessBase64(response);
                } catch { }
            }
        }

        private class ModalLayerInfo {
            public List<DynamicUINode> Pages { get; set; } = new List<DynamicUINode>();
            public string Kind { get; set; } = "sheet";
            public string OnPop { get; set; } = "";
            public string OnDismiss { get; set; } = "";
        }

        private static void ExtractNavLayers(DynamicUINode navNode, out List<DynamicUINode> windowPages, out List<ModalLayerInfo> modalLayers) {
            windowPages = new List<DynamicUINode>();
            modalLayers = new List<ModalLayerInfo>();
            if (navNode == null || navNode.Children == null || navNode.Children.Count == 0) return;

            bool hasOverlay = navNode.Props != null && navNode.Props.ContainsKey("has_overlay") && navNode.Props["has_overlay"] is bool b && b;
            int windowCount = navNode.Children.Count;
            if (navNode.Props != null && navNode.Props.ContainsKey("window_count") && int.TryParse(navNode.Props["window_count"].ToString(), out int wc)) {
                windowCount = Math.Min(navNode.Children.Count, Math.Max(0, wc));
            } else if (hasOverlay) {
                windowCount = Math.Max(1, navNode.Children.Count - 1);
            }

            for (int i = 0; i < windowCount; i++) {
                windowPages.Add(navNode.Children[i]);
            }

            string defaultPop = (navNode.Props != null && navNode.Props.ContainsKey("on_pop")) ? navNode.Props["on_pop"].ToString() : "";
            string defaultDismiss = (navNode.Props != null && navNode.Props.ContainsKey("on_dismiss")) ? navNode.Props["on_dismiss"].ToString() : defaultPop;

            var curNav = navNode;
            int curWinCount = windowCount;
            bool curHasOverlay = hasOverlay;

            while (curHasOverlay && curNav.Children != null && curNav.Children.Count > curWinCount) {
                string kind = (curNav.Props != null && curNav.Props.ContainsKey("overlay_kind")) ? curNav.Props["overlay_kind"].ToString() : "sheet";
                string onPop = (curNav.Props != null && curNav.Props.ContainsKey("on_pop")) ? curNav.Props["on_pop"].ToString() : defaultPop;
                string onDismiss = (curNav.Props != null && curNav.Props.ContainsKey("on_dismiss")) ? curNav.Props["on_dismiss"].ToString() : defaultDismiss;
                if (string.IsNullOrEmpty(onDismiss)) onDismiss = onPop;

                var firstOverlay = curNav.Children[curWinCount];
                if (firstOverlay.Name == "navigationstack") {
                    bool nextHasOverlay = firstOverlay.Props != null && firstOverlay.Props.ContainsKey("has_overlay") && firstOverlay.Props["has_overlay"] is bool nb && nb;
                    int nextCount = firstOverlay.Children != null ? firstOverlay.Children.Count : 0;
                    int nextWinCount = nextCount;
                    if (firstOverlay.Props != null && firstOverlay.Props.ContainsKey("window_count") && int.TryParse(firstOverlay.Props["window_count"].ToString(), out int nwc)) {
                        nextWinCount = Math.Min(nextCount, Math.Max(0, nwc));
                    } else if (nextHasOverlay) {
                        nextWinCount = Math.Max(1, nextCount - 1);
                    }

                    var layerPages = new List<DynamicUINode>();
                    if (firstOverlay.Children != null) {
                        for (int i = 0; i < nextWinCount; i++) {
                            layerPages.Add(firstOverlay.Children[i]);
                        }
                    }
                    string layerPop = (firstOverlay.Props != null && firstOverlay.Props.ContainsKey("on_pop")) ? firstOverlay.Props["on_pop"].ToString() : onPop;
                    string layerDismiss = (firstOverlay.Props != null && firstOverlay.Props.ContainsKey("on_dismiss")) ? firstOverlay.Props["on_dismiss"].ToString() : onDismiss;
                    if (string.IsNullOrEmpty(layerDismiss)) layerDismiss = layerPop;

                    if (layerPages.Count > 0) {
                        modalLayers.Add(new ModalLayerInfo {
                            Pages = layerPages,
                            Kind = kind,
                            OnPop = layerPop,
                            OnDismiss = layerDismiss
                        });
                    }

                    curNav = firstOverlay;
                    curWinCount = nextWinCount;
                    curHasOverlay = nextHasOverlay;
                } else {
                    var layerPages = new List<DynamicUINode>();
                    for (int i = curWinCount; i < curNav.Children.Count; i++) {
                        layerPages.Add(curNav.Children[i]);
                    }
                    if (layerPages.Count > 0) {
                        modalLayers.Add(new ModalLayerInfo {
                            Pages = layerPages,
                            Kind = kind,
                            OnPop = onPop,
                            OnDismiss = onDismiss
                        });
                    }
                    break;
                }
            }
        }

        private static bool HasTitleTextNode(DynamicUINode node) {
            if (node == null) return false;
            if (node.Name == "text" && node.Props != null && node.Props.ContainsKey("variant") && node.Props["variant"].ToString() == "title") {
                return true;
            }
            if (node.Children != null) {
                foreach (var c in node.Children) {
                    if (HasTitleTextNode(c)) return true;
                }
            }
            return false;
        }

        private UIElement BuildModalOverlay(ModalLayerInfo layer, int layerIndex, bool animateCard, bool animateInnerPage, bool isForward = true) {
            var modalBackdrop = new Grid {
                HorizontalAlignment = HorizontalAlignment.Stretch,
                VerticalAlignment = VerticalAlignment.Stretch,
                Margin = new Thickness(0, -48, 0, 0),
                Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Windows.UI.Color.FromArgb(150, 0, 0, 0))
            };
            if (!string.IsNullOrEmpty(layer.OnDismiss)) {
                string dismissAct = layer.OnDismiss;
                modalBackdrop.PointerPressed += (s, e) => {
                    if ( object.ReferenceEquals(e.OriginalSource, modalBackdrop) ) {
                        Dispatch(dismissAct, "");
                    }
                };
            }

            bool isForm = layer.Kind == "form" || layer.Kind == "dialog" || layer.Kind == "modal" || layer.Kind == "fullscreen";
            var modalCard = new Border {
                Background = Application.Current.Resources["ApplicationPageBackgroundThemeBrush"] as Microsoft.UI.Xaml.Media.Brush ?? new Microsoft.UI.Xaml.Media.SolidColorBrush(Windows.UI.Color.FromArgb(255, 32, 32, 32)),
                BorderBrush = Application.Current.Resources["SurfaceStrokeColorFlyoutBrush"] as Microsoft.UI.Xaml.Media.Brush ?? Application.Current.Resources["CardStrokeColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush ?? new Microsoft.UI.Xaml.Media.SolidColorBrush(Windows.UI.Color.FromArgb(70, 255, 255, 255)),
                BorderThickness = new Thickness(1),
                CornerRadius = new CornerRadius(8),
                HorizontalAlignment = HorizontalAlignment.Center,
                VerticalAlignment = VerticalAlignment.Center,
                Width = isForm ? 540 : 420,
                MaxWidth = isForm ? 600 : 460,
                MinHeight = isForm ? 300 : 200,
                Margin = new Thickness(24 + layerIndex * 16)
            };
            modalCard.PointerPressed += (s, e) => e.Handled = true;

            var cardInner = new Grid {
                Background = Application.Current.Resources["CardBackgroundFillColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush
            };
            cardInner.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
            cardInner.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Star) });

            var activeModalPage = layer.Pages[layer.Pages.Count - 1];
            string modalTitle = (activeModalPage.Props != null && activeModalPage.Props.ContainsKey("title")) ? activeModalPage.Props["title"].ToString() : "";
            string modalBackAction = "";
            if (layer.Pages.Count > 1) {
                modalBackAction = (activeModalPage.Props != null && activeModalPage.Props.ContainsKey("on_back")) ? activeModalPage.Props["on_back"].ToString() : layer.OnPop;
            }

            var headerBar = new Grid {
                Padding = new Thickness(8, 8, 8, 0),
                MinHeight = 40
            };
            headerBar.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });
            headerBar.ColumnDefinitions.Add(new ColumnDefinition { Width = new GridLength(1, GridUnitType.Star) });
            headerBar.ColumnDefinitions.Add(new ColumnDefinition { Width = GridLength.Auto });

            if (!string.IsNullOrEmpty(modalBackAction)) {
                string backAct = modalBackAction;
                var backBtn = new Button {
                    Width = 36,
                    Height = 32,
                    Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent),
                    BorderThickness = new Thickness(0),
                    CornerRadius = new CornerRadius(4),
                    Content = new FontIcon { Glyph = "\uE72B", FontSize = 13 }
                };
                backBtn.Click += (s, e) => Dispatch(backAct, "");
                headerBar.Children.Add(backBtn);
                Grid.SetColumn(backBtn, 0);
            }

            if (!string.IsNullOrEmpty(layer.OnDismiss)) {
                string closeAct = layer.OnDismiss;
                var closeBtn = new Button {
                    Width = 36,
                    Height = 32,
                    Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent),
                    BorderThickness = new Thickness(0),
                    CornerRadius = new CornerRadius(4),
                    Content = new FontIcon { Glyph = "\uE711", FontSize = 13 }
                };
                closeBtn.Click += (s, e) => Dispatch(closeAct, "");
                headerBar.Children.Add(closeBtn);
                Grid.SetColumn(closeBtn, 2);
            }

            cardInner.Children.Add(headerBar);
            Grid.SetRow(headerBar, 0);

            var bodyStack = new StackPanel {
                Orientation = Orientation.Vertical,
                HorizontalAlignment = HorizontalAlignment.Stretch,
                Spacing = 12
            };
            if (activeModalPage.Name == "page") {
                if (!string.IsNullOrEmpty(modalTitle) && !HasTitleTextNode(activeModalPage)) {
                    bodyStack.Children.Add(new TextBlock {
                        Text = modalTitle,
                        FontSize = 22,
                        FontWeight = Microsoft.UI.Text.FontWeights.SemiBold,
                        Margin = new Thickness(0, 0, 0, 8),
                        TextWrapping = TextWrapping.Wrap
                    });
                }
                if (activeModalPage.Children != null) {
                    foreach (var c in activeModalPage.Children) {
                        bodyStack.Children.Add(RenderNode(c));
                    }
                }
            } else {
                bodyStack.Children.Add(RenderNode(activeModalPage));
            }

            if (animateInnerPage) {
                AnimatePageIn(bodyStack, isForward);
            }

            var scrollViewer = new ScrollViewer {
                VerticalScrollBarVisibility = ScrollBarVisibility.Auto,
                Padding = new Thickness(28, 4, 28, 28),
                MaxHeight = 520,
                Content = bodyStack
            };
            cardInner.Children.Add(scrollViewer);
            Grid.SetRow(scrollViewer, 1);

            modalCard.Child = cardInner;
            modalBackdrop.Children.Add(modalCard);

            if (animateCard) {
                AnimateModalIn(modalCard);
            }
            return modalBackdrop;
        }

        private static Windows.UI.Color ShadeColor(Windows.UI.Color c, double factor) {
            byte r = (byte)Math.Clamp((int)Math.Round(c.R * factor), 0, 255);
            byte g = (byte)Math.Clamp((int)Math.Round(c.G * factor), 0, 255);
            byte b = (byte)Math.Clamp((int)Math.Round(c.B * factor), 0, 255);
            return Windows.UI.Color.FromArgb(c.A, r, g, b);
        }

        private void ApplyRuntimeThemeAndAccent(DynamicUINode node) {
            if (node == null || node.Name != "tabbar" || node.Props == null) return;
            string tm = node.Props.ContainsKey("theme_mode") ? node.Props["theme_mode"].ToString() : "system";
            string am = node.Props.ContainsKey("accent_mode") ? node.Props["accent_mode"].ToString() : "app";
            runtimeAccentMode = am;

            bool isDark = tm == "dark" || (tm == "system" && Application.Current.RequestedTheme == ApplicationTheme.Dark);
            if (this.Content is FrameworkElement rootFe) {
                var targetTheme = tm == "light" ? ElementTheme.Light : (tm == "dark" ? ElementTheme.Dark : ElementTheme.Default);
                if (rootFe.RequestedTheme != targetTheme) rootFe.RequestedTheme = targetTheme;
            }

            try {
                Windows.UI.Color restCol, hoverCol, pressedCol;
                if (am == "system") {
                    var ui = new Windows.UI.ViewManagement.UISettings();
                    restCol = ui.GetColorValue(Windows.UI.ViewManagement.UIColorType.Accent);
                    hoverCol = ui.GetColorValue(isDark ? Windows.UI.ViewManagement.UIColorType.AccentLight1 : Windows.UI.ViewManagement.UIColorType.AccentDark1);
                    pressedCol = ui.GetColorValue(isDark ? Windows.UI.ViewManagement.UIColorType.AccentLight2 : Windows.UI.ViewManagement.UIColorType.AccentDark2);
                } else {
                    restCol = ParseHexWinColor(isDark ? AppThemeConfig.DarkAccentHex : AppThemeConfig.LightAccentHex, 1.0);
                    hoverCol = ParseHexWinColor(isDark ? AppThemeConfig.DarkAccentHoverHex : AppThemeConfig.LightAccentHoverHex, 1.0);
                    pressedCol = ParseHexWinColor(isDark ? AppThemeConfig.DarkAccentPressedHex : AppThemeConfig.LightAccentPressedHex, 1.0);
                }
                runtimePrimaryHex = $"#{restCol.R:X2}{restCol.G:X2}{restCol.B:X2}";
                Application.Current.Resources["SystemAccentColor"] = restCol;
                Application.Current.Resources["SystemAccentColorLight1"] = hoverCol;
                Application.Current.Resources["SystemAccentColorLight2"] = pressedCol;
                Application.Current.Resources["SystemAccentColorDark1"] = hoverCol;
                Application.Current.Resources["SystemAccentColorDark2"] = pressedCol;
                void SetBrushColor(string key, Windows.UI.Color col) {
                    if (Application.Current.Resources.ContainsKey(key) && Application.Current.Resources[key] is Microsoft.UI.Xaml.Media.SolidColorBrush sb) {
                        try { sb.Color = col; return; } catch { }
                    }
                    Application.Current.Resources[key] = new Microsoft.UI.Xaml.Media.SolidColorBrush(col);
                }
                SetBrushColor("AccentFillColorDefaultBrush", restCol);
                SetBrushColor("AccentFillColorSecondaryBrush", hoverCol);
                SetBrushColor("AccentFillColorTertiaryBrush", pressedCol);
                SetBrushColor("SystemControlHighlightAccentBrush", restCol);
            } catch { }
        }

        private void UpdateNavigationChrome(DynamicUINode node) {
            ApplyRuntimeThemeAndAccent(node);
            if (node.Name == "tabbar" && node.Children != null && node.Children.Count > 0) {
                UpdateNavigationChrome(node.Children[0]);
                return;
            }

            string title = "";
            string onBack = "";
            string topStyle = (node.Props != null && node.Props.ContainsKey("top_style")) ? node.Props["top_style"].ToString() : "soft";

            if (node.Name == "navigationstack") {
                string onPop = (node.Props != null && node.Props.ContainsKey("on_pop")) ? node.Props["on_pop"].ToString() : "";
                ExtractNavLayers(node, out var windowPages, out _);
                if (windowPages.Count > 0) {
                    var active = windowPages[windowPages.Count - 1];
                    if (active.Props != null && active.Props.ContainsKey("title")) {
                        title = active.Props["title"].ToString();
                    }
                    if (active.Props != null && active.Props.ContainsKey("top_style")) {
                        topStyle = active.Props["top_style"].ToString();
                    }
                    if (active.Props != null && active.Props.ContainsKey("on_back")) {
                        onBack = active.Props["on_back"].ToString();
                    }
                    if (string.IsNullOrEmpty(onBack) && windowPages.Count > 1) {
                        onBack = onPop;
                    }
                }
            } else if (node.Name == "page") {
                if (node.Props != null && node.Props.ContainsKey("title")) {
                    title = node.Props["title"].ToString();
                }
                if (node.Props != null && node.Props.ContainsKey("on_back")) {
                    onBack = node.Props["on_back"].ToString();
                }
            }

            if (topStyle == "hard") {
                try {
                    AppTitleBar.Background = Application.Current.Resources["CardBackgroundFillColorDefaultBrush"] as Microsoft.UI.Xaml.Media.Brush ?? new Microsoft.UI.Xaml.Media.SolidColorBrush(Windows.UI.Color.FromArgb(255, 36, 36, 36));
                } catch { }
            } else {
                AppTitleBar.Background = new Microsoft.UI.Xaml.Media.SolidColorBrush(Microsoft.UI.Colors.Transparent);
            }

            currentBackAction = onBack;
            bool anyModalOpen = false;
            if (node.Name == "navigationstack") {
                ExtractNavLayers(node, out _, out var mLayers);
                anyModalOpen = mLayers.Count > 0;
            }
            try { TitleBarScrim.Visibility = anyModalOpen ? Visibility.Visible : Visibility.Collapsed; } catch { }
            WindowBackButton.Visibility = string.IsNullOrEmpty(onBack) ? Visibility.Collapsed : Visibility.Visible;
            WindowTitleText.Text = "My Native Kit App";
        }

        private void ProcessBase64(string b64) {
            try {
                byte[] bytes = Convert.FromBase64String(b64);
                using (var stream = new MemoryStream(bytes))
                using (var reader = new BinaryReader(stream)) {
                    byte header = reader.ReadByte();
                    if (header == 0) {
                        var node = DynamicUINode.Decode(reader);
                        UpdateNavigationChrome(node);
                        if (liveUiNode == null) {
                            liveUiNode = node;
                            RootContainer.Child = RenderNode(liveUiNode);
                        } else {
                            var oldContent = RootContainer.Child as UIElement;
                            var morphed = MorphNode(oldContent, node);
                            if (!object.ReferenceEquals(morphed, oldContent)) {
                                RootContainer.Child = morphed;
                            }
                            liveUiNode = node;
                        }
                    } else if (header == 1) {
                        if (liveUiNode != null) {
                            int patchCount = DynamicUINode.ReadU16(reader);
                            var currentRoot = liveUiNode;
                            for (int i = 0; i < patchCount; i++) {
                                byte pType = reader.ReadByte();
                                int pathLen = DynamicUINode.ReadU16(reader);
                                var path = new List<int>();
                                for (int p = 0; p < pathLen; p++) {
                                    path.Add(DynamicUINode.ReadU16(reader));
                                }
                                currentRoot = currentRoot.ApplyPatch(path, 0, pType, reader);
                            }
                            UpdateNavigationChrome(currentRoot);
                            var oldContent = RootContainer.Child as UIElement;
                            var morphed = MorphNode(oldContent, currentRoot);
                            if (!object.ReferenceEquals(morphed, oldContent)) {
                                RootContainer.Child = morphed;
                            }
                            liveUiNode = currentRoot;
                        }
                    }
                }
            } catch (Exception ex) {
                System.Diagnostics.Debug.WriteLine("ProcessBase64 error: " + ex);
            }
        }

        private async void Dispatch(string action, string payload) {
            if (isDevMode) {
                try {
                    var json = JsonSerializer.Serialize(new { action = action, payload = new { value = payload } });
                    var content = new StringContent(json, Encoding.UTF8, "application/json");
                    if (activeServerUrl != null) {
                        var resp = await client.PostAsync(activeServerUrl + "/api/dispatch", content);
                        if (resp.IsSuccessStatusCode) {
                            if (resp.Headers.TryGetValues("X-Reload-Timestamp", out var vals)) {
                                foreach (var v in vals) {
                                    if (ulong.TryParse(v, out ulong ts)) lastReloadTs = ts;
                                }
                            }
                            var b64 = await resp.Content.ReadAsStringAsync();
                            if (!string.IsNullOrEmpty(b64)) ProcessBase64(b64);
                        }
                    }
                } catch { }
            } else {
                if (appPtr != IntPtr.Zero) {
                    var payloadJson = JsonSerializer.Serialize(new { value = payload });
                    IntPtr uiPtr = dispatch_event(appPtr, action, payloadJson);
                    string b64 = Marshal.PtrToStringUTF8(uiPtr);
                    if (!string.IsNullOrEmpty(b64)) ProcessBase64(b64);
                }
            }
        }

        private void AnimatePageIn(UIElement element, bool isForward) {
            if (element == null) return;
            try {
                var storyboard = new Microsoft.UI.Xaml.Media.Animation.Storyboard();

                var fadeAnim = new Microsoft.UI.Xaml.Media.Animation.DoubleAnimation {
                    From = 0.0,
                    To = 1.0,
                    Duration = new Duration(TimeSpan.FromMilliseconds(180)),
                    EasingFunction = new Microsoft.UI.Xaml.Media.Animation.CubicEase { EasingMode = Microsoft.UI.Xaml.Media.Animation.EasingMode.EaseOut }
                };
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTarget(fadeAnim, element);
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTargetProperty(fadeAnim, "Opacity");
                storyboard.Children.Add(fadeAnim);

                var translate = new Microsoft.UI.Xaml.Media.TranslateTransform {
                    X = isForward ? 36.0 : -36.0
                };
                element.RenderTransform = translate;

                var slideAnim = new Microsoft.UI.Xaml.Media.Animation.DoubleAnimation {
                    From = isForward ? 36.0 : -36.0,
                    To = 0.0,
                    Duration = new Duration(TimeSpan.FromMilliseconds(220)),
                    EasingFunction = new Microsoft.UI.Xaml.Media.Animation.CubicEase { EasingMode = Microsoft.UI.Xaml.Media.Animation.EasingMode.EaseOut }
                };
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTarget(slideAnim, translate);
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTargetProperty(slideAnim, "X");
                storyboard.Children.Add(slideAnim);

                storyboard.Begin();
            } catch { }
        }

        private void AnimateModalIn(UIElement modalCard) {
            if (modalCard == null) return;
            try {
                var storyboard = new Microsoft.UI.Xaml.Media.Animation.Storyboard();

                var fadeAnim = new Microsoft.UI.Xaml.Media.Animation.DoubleAnimation {
                    From = 0.0,
                    To = 1.0,
                    Duration = new Duration(TimeSpan.FromMilliseconds(180)),
                    EasingFunction = new Microsoft.UI.Xaml.Media.Animation.CubicEase { EasingMode = Microsoft.UI.Xaml.Media.Animation.EasingMode.EaseOut }
                };
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTarget(fadeAnim, modalCard);
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTargetProperty(fadeAnim, "Opacity");
                storyboard.Children.Add(fadeAnim);

                var scale = new Microsoft.UI.Xaml.Media.ScaleTransform {
                    CenterX = 240,
                    CenterY = 180,
                    ScaleX = 0.95,
                    ScaleY = 0.95
                };
                modalCard.RenderTransform = scale;

                var scaleXAnim = new Microsoft.UI.Xaml.Media.Animation.DoubleAnimation {
                    To = 1.0,
                    Duration = new Duration(TimeSpan.FromMilliseconds(200)),
                    EasingFunction = new Microsoft.UI.Xaml.Media.Animation.CircleEase { EasingMode = Microsoft.UI.Xaml.Media.Animation.EasingMode.EaseOut }
                };
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTarget(scaleXAnim, scale);
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTargetProperty(scaleXAnim, "ScaleX");
                storyboard.Children.Add(scaleXAnim);

                var scaleYAnim = new Microsoft.UI.Xaml.Media.Animation.DoubleAnimation {
                    To = 1.0,
                    Duration = new Duration(TimeSpan.FromMilliseconds(200)),
                    EasingFunction = new Microsoft.UI.Xaml.Media.Animation.CircleEase { EasingMode = Microsoft.UI.Xaml.Media.Animation.EasingMode.EaseOut }
                };
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTarget(scaleYAnim, scale);
                Microsoft.UI.Xaml.Media.Animation.Storyboard.SetTargetProperty(scaleYAnim, "ScaleY");
                storyboard.Children.Add(scaleYAnim);

                storyboard.Begin();
            } catch { }
        }

                private bool isSyncingSelectorBar = false;
        private static string MapTabGlyph(string iconName) {
            switch ((iconName ?? "").ToLowerInvariant()) {
                case "house":
                case "home": return "\uE80F";
                case "bag":
                case "orders":
                case "cart": return "\uE7BF";
                case "person":
                case "profile":
                case "user": return "\uE77B";
                case "gear":
                case "settings": return "\uE713";
                default: return "\uE734";
            }
        }

        private int lastNavDepth = 1;
        private string lastPageTitle = "";
        private string lastModalSig = "";
        private int lastModalLayerCount = 0;
        private int lastTopModalDepth = 0;

                private UIElement BuildWindowsImage(DynamicUINode node) {
            string src = (node.Props != null && node.Props.ContainsKey("src")) ? node.Props["src"].ToString() : "";
            double size = (node.Props != null && node.Props.ContainsKey("size") && double.TryParse(node.Props["size"].ToString(), out double sz)) ? sz : 0;
            double w = (node.Props != null && node.Props.ContainsKey("width") && double.TryParse(node.Props["width"].ToString(), out double wd)) ? wd : size;
            double h = (node.Props != null && node.Props.ContainsKey("height") && double.TryParse(node.Props["height"].ToString(), out double ht)) ? ht : (size > 0 ? size : 180);
            double radius = (node.Props != null && node.Props.ContainsKey("corner_radius") && double.TryParse(node.Props["corner_radius"].ToString(), out double rad)) ? rad : 0;
            string mode = (node.Props != null && node.Props.ContainsKey("content_mode")) ? node.Props["content_mode"].ToString() : "cover";

            var border = new Border {
                CornerRadius = new CornerRadius(radius),
                Height = h,
                HorizontalAlignment = w > 0 ? HorizontalAlignment.Left : HorizontalAlignment.Stretch
            };
            if (w > 0) border.Width = w;

            var img = new Image {
                Stretch = (mode == "contain" || mode == "fit") ? Microsoft.UI.Xaml.Media.Stretch.Uniform : Microsoft.UI.Xaml.Media.Stretch.UniformToFill
            };
            if (!string.IsNullOrEmpty(src)) {
                try {
                    if (src.StartsWith("http://") || src.StartsWith("https://")) {
                        img.Source = new Microsoft.UI.Xaml.Media.Imaging.BitmapImage(new Uri(src));
                    } else {
                        var clean = src.Replace("./", "").TrimStart('/');
                        var full = System.IO.Path.Combine(AppContext.BaseDirectory, clean.Replace('/', System.IO.Path.DirectorySeparatorChar));
                        if (System.IO.File.Exists(full)) {
                            img.Source = new Microsoft.UI.Xaml.Media.Imaging.BitmapImage(new Uri(full));
                        } else if (activeServerUrl != null) {
                            img.Source = new Microsoft.UI.Xaml.Media.Imaging.BitmapImage(new Uri(activeServerUrl + "/" + clean));
                        }
                    }
                } catch { }
            }
            border.Child = img;
            return border;
        }

        private UIElement MorphNode(UIElement oldEl, DynamicUINode node) {
            if (oldEl == null) return RenderNode(node);
            string name = node.Name;

            if (name == "textinput" && oldEl is TextBox tbInput) {
                string newVal = (node.Props != null && node.Props.ContainsKey("value")) ? node.Props["value"].ToString() : "";
                if (tbInput.FocusState == FocusState.Unfocused && tbInput.Text != newVal) {
                    tbInput.Tag = null;
                    tbInput.Text = newVal;
                }
                if (node.Props != null && node.Props.ContainsKey("on_change")) {
                    tbInput.Tag = node.Props["on_change"].ToString();
                }
                return tbInput;
            }

            if (name == "image" && oldEl is Border imgBorder) {
                return imgBorder;
            }

            if (name == "icon" && oldEl is FontIcon oldIcon) {
                return oldIcon;
            }

            if (name == "switch" && oldEl is CheckBox oldSw) {
                return oldSw;
            }

            if (name == "tabbar" && oldEl is Grid oldTabGrid && oldTabGrid.Children.Count >= 2) {
                int activeIdx = 0;
                if (node.Props != null && node.Props.ContainsKey("active_index") && int.TryParse(node.Props["active_index"].ToString(), out int aIdx)) {
                    activeIdx = aIdx;
                }
                bool tabSwitched = false;
                bool isForwardTab = true;
                if (oldTabGrid.Children[0] is SelectorBar selBar) {
                    int prevIdx = selBar.SelectedItem != null ? selBar.Items.IndexOf(selBar.SelectedItem) : activeIdx;
                    if (prevIdx < 0) prevIdx = activeIdx;
                    if (activeIdx >= 0 && activeIdx < selBar.Items.Count && prevIdx != activeIdx) {
                        tabSwitched = true;
                        isForwardTab = activeIdx > prevIdx;
                        isSyncingSelectorBar = true;
                        selBar.SelectedItem = selBar.Items[activeIdx];
                        isSyncingSelectorBar = false;
                    }
                    if (node.Props != null && node.Props.ContainsKey("tabs") && node.Props["tabs"] is System.Text.Json.JsonElement elem && elem.ValueKind == System.Text.Json.JsonValueKind.Array) {
                        int i = 0;
                        foreach (var item in elem.EnumerateArray()) {
                            if (i < selBar.Items.Count) {
                                string action = item.TryGetProperty("action", out var ap) ? ap.GetString() : "";
                                string title = item.TryGetProperty("title", out var tp) ? tp.GetString() : "";
                                string winIc = item.TryGetProperty("windows_icon", out var wp) ? wp.GetString() : "";
                                selBar.Items[i].Tag = action;
                                selBar.Items[i].Text = title;
                                if (!string.IsNullOrEmpty(winIc) && selBar.Items[i].Icon is FontIcon fi) fi.Glyph = winIc;
                            }
                            i++;
                        }
                    }
                }
                if (node.Children != null && node.Children.Count > 0) {
                    var oldChild = oldTabGrid.Children[1];
                    var morphedChild = MorphNode(oldChild, node.Children[0]);
                    if (!object.ReferenceEquals(morphedChild, oldChild)) {
                        oldTabGrid.Children.RemoveAt(1);
                        oldTabGrid.Children.Add(morphedChild);
                        if (morphedChild is FrameworkElement fe) Grid.SetRow(fe, 1);
                    }
                    if (tabSwitched) {
                        AnimatePageIn(oldTabGrid.Children[1], isForwardTab);
                    }
                }
                return oldTabGrid;
            }

            if (name == "navigationstack" && oldEl is Grid oldNavGrid) {
                if (node.Children != null && node.Children.Count > 0) {
                    ExtractNavLayers(node, out var windowPages, out var modalLayers);
                    var activeWin = windowPages.Count > 0 ? windowPages[windowPages.Count - 1] : node.Children[0];
                    int newWinDepth = windowPages.Count;
                    string currentWinTitle = (activeWin.Props != null && activeWin.Props.ContainsKey("title")) ? activeWin.Props["title"].ToString() : "";
                    if (string.IsNullOrEmpty(lastPageTitle)) lastPageTitle = currentWinTitle;

                    var sigBuilder = new StringBuilder();
                    foreach (var l in modalLayers) {
                        var topP = l.Pages[l.Pages.Count - 1];
                        string tTitle = (topP.Props != null && topP.Props.ContainsKey("title")) ? topP.Props["title"].ToString() : "";
                        sigBuilder.Append($"{l.Kind}:{l.Pages.Count}:{tTitle}|");
                    }
                    string newModalSig = sigBuilder.ToString();
                    int newModalLayerCount = modalLayers.Count;
                    int newTopModalDepth = newModalLayerCount > 0 ? modalLayers[newModalLayerCount - 1].Pages.Count : 0;

                    bool winChanged = (newWinDepth != lastNavDepth) || (currentWinTitle != lastPageTitle);
                    bool modalChanged = (newModalSig != lastModalSig);

                    if (!winChanged && !modalChanged && oldNavGrid.Children.Count == 1 + modalLayers.Count) {
                        MorphNode(oldNavGrid.Children[0], activeWin);
                        for (int i = 0; i < modalLayers.Count; i++) {
                            oldNavGrid.Children.RemoveAt(1 + i);
                            oldNavGrid.Children.Insert(1 + i, BuildModalOverlay(modalLayers[i], i, false, false, true));
                        }
                        return oldNavGrid;
                    }

                    bool isWinForward = newWinDepth >= lastNavDepth;
                    int prevModalLayerCount = lastModalLayerCount;
                    int prevTopModalDepth = lastTopModalDepth;

                    lastNavDepth = newWinDepth;
                    lastPageTitle = currentWinTitle;
                    lastModalSig = newModalSig;
                    lastModalLayerCount = newModalLayerCount;
                    lastTopModalDepth = newTopModalDepth;

                    oldNavGrid.Children.Clear();
                    var pageEl = RenderNode(activeWin);
                    oldNavGrid.Children.Add(pageEl);
                    if (winChanged && modalLayers.Count == 0) {
                        AnimatePageIn(pageEl, isWinForward);
                    }

                    for (int i = 0; i < modalLayers.Count; i++) {
                        bool isTop = (i == modalLayers.Count - 1);
                        bool animateCard = isTop && (modalLayers.Count > prevModalLayerCount);
                        bool isModalForward = newTopModalDepth >= prevTopModalDepth;
                        bool animateInnerPage = isTop && (modalLayers.Count == prevModalLayerCount) && (newTopModalDepth != prevTopModalDepth);
                        oldNavGrid.Children.Add(BuildModalOverlay(modalLayers[i], i, animateCard, animateInnerPage, isModalForward));
                    }
                }
                return oldNavGrid;
            }

            StackPanel pPanel = null;
            if (name == "page") {
                if (oldEl is ScrollViewer pScroll && pScroll.Content is StackPanel sp) {
                    pPanel = sp;
                }
            }
            if (pPanel != null) {
                int offset = 0;
                if (pPanel.Children.Count > 0 && pPanel.Children[0] is FrameworkElement fe0 && (fe0.Tag as string == "__page_title__" || fe0.Tag as string == "__page_header__")) {
                    var freshPage = RenderNode(node) as ScrollViewer;
                    if (freshPage != null && freshPage.Content is StackPanel freshSp && freshSp.Children.Count > 0) {
                        var freshHdr = freshSp.Children[0];
                        freshSp.Children.RemoveAt(0);
                        pPanel.Children.RemoveAt(0);
                        pPanel.Children.Insert(0, freshHdr);
                    }
                    offset = 1;
                }
                var bodyChildren = new List<DynamicUINode>();
                if (node.Children != null) {
                    foreach (var c in node.Children) {
                        if (c.Name != "toolbartop" && c.Name != "toolbarbottom") bodyChildren.Add(c);
                    }
                }
                int len = bodyChildren.Count;
                for (int i = 0; i < len; i++) {
                    int targetIdx = i + offset;
                    if (targetIdx < pPanel.Children.Count) {
                        var morphed = MorphNode(pPanel.Children[targetIdx], bodyChildren[i]);
                        if (!object.ReferenceEquals(morphed, pPanel.Children[targetIdx])) {
                            pPanel.Children.RemoveAt(targetIdx);
                            pPanel.Children.Insert(targetIdx, morphed);
                        }
                    } else {
                        pPanel.Children.Add(RenderNode(bodyChildren[i]));
                    }
                }
                while (pPanel.Children.Count > len + offset) {
                    pPanel.Children.RemoveAt(pPanel.Children.Count - 1);
                }
                return oldEl;
            }

            if (name == "text" && oldEl is TextBlock tb) {
                if (node.Props != null && node.Props.ContainsKey("label")) {
                    string newLbl = node.Props["label"].ToString();
                    if (tb.Text != newLbl) tb.Text = newLbl;
                }
                if (node.Props != null && node.Props.ContainsKey("color")) {
                    var fg = ResolveAppBrush(node.Props["color"].ToString());
                    if (fg != null) tb.Foreground = fg;
                }
                return tb;
            }

            if (name == "button" && oldEl is Button btn) {
                string widthProp = (node.Props != null && node.Props.ContainsKey("width")) ? node.Props["width"].ToString() : "";
                var targetAlign = (widthProp == "fill") ? HorizontalAlignment.Stretch : HorizontalAlignment.Left;
                if (btn.HorizontalAlignment != targetAlign) btn.HorizontalAlignment = targetAlign;
                string btnVar = (node.Props != null && node.Props.ContainsKey("variant")) ? node.Props["variant"].ToString() : "";
                if (btnVar == "primary") {
                    var priBrush = ResolveAppBrush("primary");
                    if (priBrush != null) btn.Background = priBrush;
                }
                if (node.Props != null && node.Props.ContainsKey("action")) {
                    btn.Tag = node.Props["action"].ToString();
                }
                if (node.Children != null && node.Children.Count > 0) {
                    var oldBtnChild = btn.Content as UIElement;
                    var morphedBtnChild = MorphNode(oldBtnChild, node.Children[0]);
                    if (!object.ReferenceEquals(morphedBtnChild, oldBtnChild)) {
                        btn.Content = morphedBtnChild;
                    }
                }
                return btn;
            }

            if (name == "vstack" && oldEl is StackPanel spV && spV.Orientation == Orientation.Vertical) {
                MorphChildren(spV.Children, node);
                return spV;
            }

            if (name == "hstack" && oldEl is StackPanel spH && spH.Orientation == Orientation.Horizontal) {
                MorphChildren(spH.Children, node);
                return spH;
            }

            return RenderNode(node);
        }

        private void MorphChildren(UIElementCollection childrenCol, DynamicUINode node) {
            if (node.Children == null) {
                childrenCol.Clear();
                return;
            }
            int len = node.Children.Count;
            for (int i = 0; i < len; i++) {
                if (i < childrenCol.Count) {
                    var morphed = MorphNode(childrenCol[i], node.Children[i]);
                    if (morphed != childrenCol[i]) {
                        childrenCol.RemoveAt(i);
                        childrenCol.Insert(i, morphed);
                    }
                } else {
                    childrenCol.Add(RenderNode(node.Children[i]));
                }
            }
            while (childrenCol.Count > len) {
                childrenCol.RemoveAt(childrenCol.Count - 1);
            }
        }

        
        

private UIElement RenderNode(DynamicUINode node) {
            string name = node.Name;
            switch (name) {
                
case "button": {
    var btn = new Button();
    if (node.Props != null && node.Props.ContainsKey("variant")) {
        string v = node.Props["variant"].ToString();
        if (v == "primary") {
            try { btn.Style = Application.Current.Resources["AccentButtonStyle"] as Style; } catch { }
        }
    }
    string widthProp = (node.Props != null && node.Props.ContainsKey("width")) ? node.Props["width"].ToString() : "";
    if (widthProp == "fill") {
        btn.HorizontalAlignment = HorizontalAlignment.Stretch;
    } else {
        btn.HorizontalAlignment = HorizontalAlignment.Left;
    }
    btn.HorizontalContentAlignment = HorizontalAlignment.Center;
    if (node.Props != null && node.Props.ContainsKey("color")) {
        var brush = ResolveAppBrush(node.Props["color"].ToString());
        if (brush != null) btn.Background = brush;
    }
    if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double bOp)) {
        btn.Opacity = bOp / 100.0;
    }
    if (node.Children != null && node.Children.Count > 0) {
        btn.Content = RenderNode(node.Children[0]);
    }
    if (node.Props != null && node.Props.ContainsKey("action")) {
        btn.Tag = node.Props["action"].ToString();
        btn.Click += (s, e) => {
            if (btn.Tag is string act) Dispatch(act, "");
        };
    }
    return btn;
}


                case "text":
                    var tb = new TextBlock { TextWrapping = TextWrapping.Wrap };
                    if (node.Props != null && node.Props.ContainsKey("label")) {
                        tb.Text = node.Props["label"].ToString();
                    }
                    string ts = (node.Props != null && node.Props.ContainsKey("style")) ? node.Props["style"].ToString() : "body";
                    bool emphasized = (node.Props != null && node.Props.ContainsKey("emphasized") && node.Props["emphasized"] is bool emp && emp);
                    switch (ts) {
                        case "body":
                            tb.FontSize = 17;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "caption1":
                            tb.FontSize = 12;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "caption2":
                            tb.FontSize = 11;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "footnote":
                            tb.FontSize = 13;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "headline":
                            tb.FontSize = 17;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.Bold : Microsoft.UI.Text.FontWeights.SemiBold;
                            break;
                        case "large_title":
                            tb.FontSize = 34;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.Bold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "subhead":
                            tb.FontSize = 15;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "title1":
                            tb.FontSize = 28;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.Bold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "title2":
                            tb.FontSize = 22;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.Bold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        case "title3":
                            tb.FontSize = 20;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                        default:
                            tb.FontSize = 17;
                            tb.FontWeight = emphasized ? Microsoft.UI.Text.FontWeights.SemiBold : Microsoft.UI.Text.FontWeights.Normal;
                            break;
                    }
                    if (node.Props != null && node.Props.ContainsKey("color")) {
                        var fg = ResolveAppBrush(node.Props["color"].ToString());
                        if (fg != null) tb.Foreground = fg;
                    }
                    if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double tbOp)) {
                        tb.Opacity = tbOp / 100.0;
                    }
                    return tb;


                case "icon":
                    string iconKey = (node.Props != null && node.Props.ContainsKey("name")) ? node.Props["name"].ToString() : "sparkles";
                    double iconSize = (node.Props != null && node.Props.ContainsKey("size") && double.TryParse(node.Props["size"].ToString(), out double iSz)) ? iSz : 20.0;
                    string glyph = "\uE734";
                    switch (iconKey) {
                        case "arrow_left": glyph = "\uE72B"; break;
                        case "arrow_right": glyph = "\uE72A"; break;
                        case "bell": glyph = "\uEA8F"; break;
                        case "bolt": glyph = "\uE945"; break;
                        case "brand_spark": glyph = "\uE734"; break;
                        case "cart": glyph = "\uE7BF"; break;
                        case "check": glyph = "\uE73E"; break;
                        case "check_circle": glyph = "\uE73E"; break;
                        case "chevron_left": glyph = "\uE76B"; break;
                        case "chevron_right": glyph = "\uE76C"; break;
                        case "close": glyph = "\uE711"; break;
                        case "edit": glyph = "\uE70F"; break;
                        case "heart": glyph = "\uEB51"; break;
                        case "home": glyph = "\uE80F"; break;
                        case "info": glyph = "\uE946"; break;
                        case "lock": glyph = "\uE72E"; break;
                        case "mail": glyph = "\uE715"; break;
                        case "orders": glyph = "\uE7BF"; break;
                        case "plus": glyph = "\uE710"; break;
                        case "refresh": glyph = "\uE72C"; break;
                        case "search": glyph = "\uE721"; break;
                        case "settings": glyph = "\uE713"; break;
                        case "share": glyph = "\uE72D"; break;
                        case "sparkles": glyph = "\uE734"; break;
                        case "star": glyph = "\uE734"; break;
                        case "trash": glyph = "\uE74D"; break;
                        case "user": glyph = "\uE77B"; break;
                        case "warning": glyph = "\uE7BA"; break;
                        default: glyph = "\uE734"; break;
                    }
                    var fi = new FontIcon { Glyph = glyph, FontSize = iconSize };
                    if (node.Props != null && node.Props.ContainsKey("color")) {
                        var fg = ResolveAppBrush(node.Props["color"].ToString());
                        if (fg != null) fi.Foreground = fg;
                    }
                    if (node.Props != null && node.Props.ContainsKey("opacity") && double.TryParse(node.Props["opacity"].ToString(), out double fiOp)) {
                        fi.Opacity = fiOp / 100.0;
                    }
                    return fi;


case "hstack":
    var hp = new StackPanel { Orientation = Orientation.Horizontal };
    if (node.Props != null && node.Props.ContainsKey("spacing") && double.TryParse(node.Props["spacing"].ToString(), out double hSpace)) {
        hp.Spacing = hSpace;
    }
    ApplyWindowsContainerStyle(hp, node);
    if (node.Children != null) {
        foreach (var c in node.Children) hp.Children.Add(RenderNode(c));
    }
    return hp;


case "vstack":
    var vp = new StackPanel { Orientation = Orientation.Vertical };
    if (node.Props != null && node.Props.ContainsKey("spacing") && double.TryParse(node.Props["spacing"].ToString(), out double vSpace)) {
        vp.Spacing = vSpace;
    }
    ApplyWindowsContainerStyle(vp, node);
    if (node.Children != null) {
        foreach (var c in node.Children) vp.Children.Add(RenderNode(c));
    }
    return vp;


case "switch":
    var sw = new CheckBox();
    if (node.Props != null && node.Props.ContainsKey("is_on") && node.Props["is_on"] is bool b) {
        sw.IsChecked = b;
    }
    if (node.Props != null && node.Props.ContainsKey("on_toggle")) {
        string action = node.Props["on_toggle"].ToString();
        sw.Checked += (s, e) => Dispatch(action, "true");
        sw.Unchecked += (s, e) => Dispatch(action, "false");
    }
    return sw;



case "navigationstack":
    var navGrid = new Grid { HorizontalAlignment = HorizontalAlignment.Stretch, VerticalAlignment = VerticalAlignment.Stretch };
    if (node.Children != null && node.Children.Count > 0) {
        ExtractNavLayers(node, out var windowPages, out var modalLayers);

        if (windowPages.Count > 0) {
            var pageEl = RenderNode(windowPages[windowPages.Count - 1]);
            navGrid.Children.Add(pageEl);
            AnimatePageIn(pageEl, true);
        }

        for (int i = 0; i < modalLayers.Count; i++) {
            var modalOverlay = BuildModalOverlay(modalLayers[i], i, true, false, true);
            navGrid.Children.Add(modalOverlay);
        }
    }
    return navGrid;



case "page":
    var pageScroll = new ScrollViewer {
        HorizontalScrollBarVisibility = ScrollBarVisibility.Disabled,
        VerticalScrollBarVisibility = ScrollBarVisibility.Auto,
        HorizontalAlignment = HorizontalAlignment.Stretch,
        VerticalAlignment = VerticalAlignment.Stretch
    };
    var pageContent = new StackPanel {
        HorizontalAlignment = HorizontalAlignment.Stretch,
        Margin = new Thickness(16, 16, 16, 24)
    };
    if (node.Props != null && node.Props.ContainsKey("title")) {
        string pTitle = node.Props["title"].ToString();
        if (!string.IsNullOrEmpty(pTitle)) {
            var titleTb = new TextBlock {
                Text = pTitle,
                FontSize = 28,
                FontWeight = Microsoft.UI.Text.FontWeights.SemiBold,
                Margin = new Thickness(0, 0, 0, 16),
                Tag = "__page_title__"
            };
            pageContent.Children.Add(titleTb);
        }
    }
    if (node.Children != null) {
        foreach (var c in node.Children) pageContent.Children.Add(RenderNode(c));
    }
    pageScroll.Content = pageContent;
    return pageScroll;


                
/* END_PRIMITIVES */
case "tabbar": {
                    var tabGrid = new Grid { HorizontalAlignment = HorizontalAlignment.Stretch, VerticalAlignment = VerticalAlignment.Stretch };
                    tabGrid.RowDefinitions.Add(new RowDefinition { Height = GridLength.Auto });
                    tabGrid.RowDefinitions.Add(new RowDefinition { Height = new GridLength(1, GridUnitType.Star) });

                    var selectorBar = new SelectorBar {
                        HorizontalAlignment = HorizontalAlignment.Left,
                        Margin = new Thickness(12, 2, 12, 2)
                    };

                    int activeIdx = 0;
                    if (node.Props != null && node.Props.ContainsKey("active_index") && int.TryParse(node.Props["active_index"].ToString(), out int aIdx)) {
                        activeIdx = aIdx;
                    }

                    if (node.Props != null && node.Props.ContainsKey("tabs")) {
                        var rawTabs = node.Props["tabs"];
                        if (rawTabs is System.Text.Json.JsonElement elem && elem.ValueKind == System.Text.Json.JsonValueKind.Array) {
                            int i = 0;
                            foreach (var item in elem.EnumerateArray()) {
                                string title = item.TryGetProperty("title", out var tp) ? tp.GetString() : "";
                                string icon = item.TryGetProperty("windows_icon", out var wp) ? wp.GetString() : (item.TryGetProperty("icon", out var ip) ? ip.GetString() : "");
                                string action = item.TryGetProperty("action", out var ap) ? ap.GetString() : "";

                                var sbItem = new SelectorBarItem {
                                    Text = title,
                                    Icon = new FontIcon { Glyph = (icon != null && icon.Length == 1 && icon[0] >= 0xE000) ? icon : MapTabGlyph(icon) },
                                    Tag = action,
                                    IsSelected = (i == activeIdx)
                                };
                                selectorBar.Items.Add(sbItem);
                                i++;
                            }
                        }
                    }

                    selectorBar.SelectionChanged += (s, e) => {
                        if (isSyncingSelectorBar) return;
                        if (selectorBar.SelectedItem is SelectorBarItem selected && selected.Tag is string act && !string.IsNullOrEmpty(act)) {
                            Dispatch(act, "");
                        }
                    };

                    tabGrid.Children.Add(selectorBar);
                    Grid.SetRow(selectorBar, 0);

                    if (node.Children != null && node.Children.Count > 0) {
                        var content = RenderNode(node.Children[0]);
                        tabGrid.Children.Add(content);
                        if (content is FrameworkElement feContent) Grid.SetRow(feContent, 1);
                    }
                    return tabGrid;
                }

                case "list":
                                case "if":
                    var sp = new StackPanel();
                    if (node.Children != null) {
                        foreach (var c in node.Children) {
                            sp.Children.Add(RenderNode(c));
                        }
                    }
                    return sp;
                default:
                    return RenderGeneratedPrimitive(node);
            }
        }
    }
}
