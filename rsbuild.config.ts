import { defineConfig } from "@rsbuild/core";
import { pluginNodePolyfill } from "@rsbuild/plugin-node-polyfill";
import { pluginReact } from "@rsbuild/plugin-react";
import { tanstackRouter } from "@tanstack/router-plugin/rspack";

export default defineConfig({
	plugins: [pluginReact(), pluginNodePolyfill()],
	resolve: {
		alias: {
			"@": "./src",
			// 在构建时使用全局 Excalidraw 对象，避免打包本地包
			"@mg-chao/excalidraw": "window.ExcalidrawLib",
		},
	},
	output: {
		cleanDistPath: true,
	},
	performance: {
		chunkSplit: {
			strategy: "split-by-module",
		},
	},
	html: {
		tags: [
			{
				tag: "script",
				attrs: {
					src:
						import.meta.env.PUBLIC_ONLINE_STATUS === "true"
							? "/scripts/excalidraw.js"
							: "/scripts/excalidraw.offline.js",
				},
			},
			{
				tag: "script",
				attrs: {
					src: "/scripts/markdownItFix.js",
				},
			},
		],
	},
	tools: {
		swc: {
			jsc: {
				experimental: {
					plugins: [["@swc/plugin-styled-jsx", {}]],
				},
			},
		},
		rspack: {
			plugins: [
				tanstackRouter({
					target: "react",
					autoCodeSplitting: true,
				}),
			],
			optimization: {},
			externals: {
				"@mg-chao/excalidraw": "window.ExcalidrawLib",
				"@mg-chao/excalidraw/types": "window.ExcalidrawLib",
				"@mg-chao/excalidraw/element/types": "window.ExcalidrawLib",
			},
		},
	},
});
