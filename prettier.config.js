/**
 * @see https://prettier.io/docs/en/configuration.html
 * @type {import("prettier").Config}
 */
export default {
  plugins: [
    "prettier-plugin-svelte",
    "prettier-plugin-tailwindcss",
    "@trivago/prettier-plugin-sort-imports",
  ],
  trailingComma: "es5",
  tabWidth: 2,
  semi: true,
  singleQuote: false,
  importOrder: [
    "^(svelte/(.*))$",
    "<THIRD_PARTY_MODULES>",
    "^lucide-svelte/(.*)",
    "^@modules/(.*)",
    "^@components/(.*)",
    "^@routes/(.*)",
    "^@lib/(?!types)(.*)",
    "^@lib/types/(.*)",
    "^@/(.*)",
    "^.(.?)/(?!.*\\.css$).*$",
    "^.*\\.css$",
  ],
  importOrderSeparation: true,
  importOrderSortSpecifiers: true,
  overrides: [
    {
      files: "*.svelte",
      options: {
        parser: "svelte",
      },
    },
    {
      files: "*.svelte.ts",
      options: {
        parser: "svelte",
      },
    },
  ],
};
