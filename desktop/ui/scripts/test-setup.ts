import { compileModule } from "svelte/compiler";

const globals = globalThis as unknown as Record<string, unknown>;
globals.window ??= globalThis;
globals.location ??= new URL("http://localhost/");

const transpiler = new Bun.Transpiler({ loader: "ts" });

Bun.plugin({
  name: "svelte-runes-ts",
  setup(build) {
    build.onLoad({ filter: /\.svelte\.ts$/ }, async ({ path }) => {
      const source = transpiler.transformSync(await Bun.file(path).text());
      const { js } = compileModule(source, { filename: path, generate: "client" });
      return { contents: js.code, loader: "js" };
    });
  },
});
