export const loadWasm = async () => {
	const wasm = await import('./wasm/portfolio_wasm.js');
	await wasm.default();
	return wasm;
}
