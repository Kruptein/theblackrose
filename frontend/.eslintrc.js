module.exports = {
    root: true,
    env: {
        node: true,
    },
    extends: [
        "plugin:vue/vue3-essential",
        "eslint:recommended",
        "@vue/typescript/recommended",
        "@vue/prettier",
        "@vue/prettier/@typescript-eslint",
        "plugin:import/typescript",
    ],
    rules: {
        "@typescript-eslint/await-thenable": 2,
        "@typescript-eslint/consistent-type-assertions": [2, { assertionStyle: "as" }],
        "@typescript-eslint/explicit-function-return-type": [2, { allowExpressions: true }],
        "@typescript-eslint/explicit-member-accessibility": [2, { accessibility: "no-public" }],
        "@typescript-eslint/no-angle-bracket-type-assertion": "off",
        "@typescript-eslint/no-empty-function": "off",
        "@typescript-eslint/no-explicit-any": "off",
        "@typescript-eslint/no-non-null-assertion": "off",
        "@typescript-eslint/no-unused-vars": [
            process.env.GITHUB_ACTION === undefined ? 1 : 2,
            { argsIgnorePattern: "^_", varsIgnorePattern: "^_" },
        ],
        "@typescript-eslint/no-use-before-define": 0,
        "@typescript-eslint/require-await": "error",
        "@typescript-eslint/strict-boolean-expressions": 2,
        "prefer-const": [process.env.GITHUB_ACTION === undefined ? 1 : 2],
        "import/no-unused-modules": [process.env.GITHUB_ACTION === undefined ? 0 : 2, { unusedExports: true }],
        "import/order": [
            process.env.GITHUB_ACTION === undefined ? 1 : 2,
            {
                alphabetize: { order: "asc", caseInsensitive: true },
                "newlines-between": "always",
                pathGroups: [{ pattern: "@/**", group: "parent", position: "before" }],
            },
        ],
        "vue/no-unused-components": [process.env.GITHUB_ACTION === undefined ? 1 : 2],
        // "no-console": process.env.NODE_ENV === "production" ? "error" : "off",
        "no-console": "off",
        "no-constant-condition": "off",
        "no-debugger": process.env.NODE_ENV === "production" ? "error" : "off",
        "no-dupe-class-members": "off",
        "no-empty-function": "off",
        "no-unused-vars": "off",
        "prettier/prettier": [process.env.GITHUB_ACTION === undefined ? 1 : 2],
    },
    overrides: [
        {
            files: ["**/__tests__/*.{j,t}s?(x)", "**/tests/unit/**/*.spec.{j,t}s?(x)"],
            env: {
                jest: true,
            },
        },
    ],
    plugins: ["import"],
    settings: {
        "import/resolver": {
            typescript: {
                extensions: [".ts", ".d.ts", ".vue"],
            },
        },
        "import/extensions": [".ts", ".vue"],
    },
    parserOptions: {
        parser: "@typescript-eslint/parser",
        ecmaFeatures: {
            jsx: false,
        },
        project: "./tsconfig.json",
        tsconfigRootDir: __dirname,
    },
};
