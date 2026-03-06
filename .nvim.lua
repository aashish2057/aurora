local ok, conform = pcall(require, "conform")
if ok then
	conform.formatters_by_ft["rust"] = { "rustfmt" }
end

vim.lsp.config["rust_analyzer"] = {
	settings = {
		["rust-analyzer"] = {
			checkOnSave = true,
			check = {
				command = "clippy",
			},
		},
	},
}

vim.lsp.enable("rust_analyzer")
