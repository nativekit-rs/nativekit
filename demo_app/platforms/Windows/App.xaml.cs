using Microsoft.UI.Xaml;
using System;
using System.IO;

namespace CargoNativeApp {
    public partial class App : Application {
        private Window m_window;

        public App() {
            this.InitializeComponent();
            this.UnhandledException += (sender, e) => {
                string error = $"[NativeKit Error] {e.Message}\n{e.Exception}\n{e.Exception?.StackTrace}";
                Console.WriteLine(error);
                try {
                    File.WriteAllText("crash.log", error);
                } catch { }
            };
        }

        protected override void OnLaunched(Microsoft.UI.Xaml.LaunchActivatedEventArgs args) {
            m_window = new MainWindow();
            m_window.Activate();
        }
    }
}
