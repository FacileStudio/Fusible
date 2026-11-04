import type { CapacitorConfig } from "@capacitor/cli";

const config: CapacitorConfig = {
  appId: "studio.facile.fusible",
  appName: "Fusible",
  webDir: "build",
  server: { androidScheme: "https" },
  android: { allowMixedContent: false },
  plugins: { CapacitorHttp: { enabled: true } },
};

export default config;
