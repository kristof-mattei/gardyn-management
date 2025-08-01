import js from "@eslint/js";
import stylistic from "@stylistic/eslint-plugin";
import tsParser from "@typescript-eslint/parser";
import love from "eslint-config-love";
import importPlugin from "eslint-plugin-import";
import nPlugin from "eslint-plugin-n";
import perfectionist from "eslint-plugin-perfectionist";
import prettier from "eslint-plugin-prettier/recommended";
import promise from "eslint-plugin-promise";
import reactPlugin from "eslint-plugin-react";
import reactHookFormPlugin from "eslint-plugin-react-hook-form";
import reactRefreshPlugin from "eslint-plugin-react-refresh";
import eslintPluginUnicorn from "eslint-plugin-unicorn";
import tseslint from "typescript-eslint";

const sharedRules = {
    "arrow-body-style": ["error", "always"],
    complexity: ["off"],
    curly: ["error", "all"],
    "eol-last": ["error", "always"],
    eqeqeq: ["error", "always"],
    "no-alert": ["off"],
    "no-console": ["off"],
    "max-len": ["off"],
    "max-nested-callbacks": ["off"],
    "no-extra-semi": ["off"],
    "no-param-reassign": ["off"],
    "no-restricted-imports": [
        "error",
        {
            patterns: [".*"],
        },
    ],
    "no-restricted-syntax": ["error", "DebuggerStatement", "LabeledStatement", "WithStatement"],
    "no-shadow": ["error"],
    "no-underscore-dangle": ["off"],
    "no-unused-expressions": ["error"],
    "no-useless-constructor": ["off"],
    "object-shorthand": ["error", "always"],
    "prefer-template": ["error"],
    quotes: [
        "error",
        "double",
        {
            allowTemplateLiterals: false,
            avoidEscape: true,
        },
    ],
    "require-await": ["error"],
    "sort-imports": [
        "error",
        {
            ignoreDeclarationSort: true,
        },
    ],

    "sort-keys": ["off"],
    "unicorn/no-null": ["off"],
    "unicorn/prefer-ternary": ["off"],

    "import/extensions": [
        "error",
        "ignorePackages",
        {
            json: "always",
            ts: "always",
            tsx: "always",
        },
    ],
    "import/newline-after-import": ["error"],
    "import/no-cycle": ["off"],
    "import/no-extraneous-dependencies": ["off"],
    "import/no-relative-packages": ["error"],
    "import/no-unresolved": ["error"],
    "import/order": [
        "error",
        {
            alphabetize: { caseInsensitive: true, order: "asc" },
            "newlines-between": "always-and-inside-groups",
        },
    ],
    "import/prefer-default-export": ["off"],
};

export default tseslint.config(
    js.configs.recommended,
    {
        ignores: ["dist/**", "reports/**", "coverage/**"],
    },
    eslintPluginUnicorn.configs["all"],
    {
        languageOptions: {
            ...reactPlugin.configs.flat["jsx-runtime"].languageOptions,
            parser: tsParser,
            parserOptions: {
                ecmaVersion: "latest",
                projectService: true,
                sourceType: "module", // Allows for the use of imports
                tsconfigRootDir: import.meta.dirname,
            },
        },
        ...reactPlugin.configs.flat["jsx-runtime"],
        plugins: {
            import: importPlugin,
            "react-refresh": reactRefreshPlugin,
            "react-hook-form": reactHookFormPlugin,
        },
        settings: {
            "import/resolver": {
                node: {},
                typescript: {
                    alwaysTryTypes: true,
                },
            },
            react: {
                version: "detect",
            },
        },
        extends: [eslintPluginUnicorn.configs["recommended"]],
        rules: {
            ...importPlugin.configs.recommended.rules,
            ...importPlugin.configs.react.rules,
            ...reactHookFormPlugin.configs.recommended.rules,

            "react-refresh/only-export-components": "error",

            ...sharedRules,
        },
    },
    {
        files: ["**/*.ts", "**/*.tsx"],
        ignores: ["**/*.mjs"],
        languageOptions: {
            parser: tsParser,
            parserOptions: {
                ecmaVersion: "latest",
                project: "./tsconfig.json",
                projectService: true,
                sourceType: "module", // Allows for the use of imports
                tsconfigRootDir: import.meta.dirname,
            },
        },
        plugins: {
            "@stylistic/ts": stylistic,
            import: importPlugin,
            n: nPlugin,
            promise,
            perfectionist,
        },
        extends: [
            ...tseslint.configs.strictTypeChecked,
            ...tseslint.configs.recommendedTypeChecked,
            ...tseslint.configs.stylisticTypeChecked,
            love,
        ],
        settings: {
            "import/resolver": {
                node: {},
                typescript: {
                    alwaysTryTypes: true,
                },
            },
        },
        rules: {
            ...importPlugin.configs.typescript.rules,
            ...importPlugin.configs.react.rules,
            ...importPlugin.configs.recommended.rules,

            ...sharedRules,

            "no-restricted-imports": ["off"],

            "@stylistic/ts/no-extra-semi": ["error"],

            "@typescript-eslint/consistent-type-imports": [
                "error",
                {
                    disallowTypeAnnotations: false,
                    fixStyle: "separate-type-imports",
                    prefer: "type-imports",
                },
            ], // different than love
            "@typescript-eslint/prefer-destructuring": ["off"],
            "@typescript-eslint/explicit-member-accessibility": ["error"],
            "@typescript-eslint/explicit-module-boundary-types": ["error"],
            "@typescript-eslint/member-ordering": ["error"],
            "@typescript-eslint/no-empty-object-type": ["error"], // stricter than love
            "@typescript-eslint/no-explicit-any": ["error", { fixToUnknown: true, ignoreRestArgs: false }], // stricter than love
            "@typescript-eslint/no-extraneous-class": ["error"], // stricter than love
            "@typescript-eslint/no-magic-numbers": ["off"],
            "@typescript-eslint/no-shadow": ["error"],
            "@typescript-eslint/no-unused-expressions": [
                "error",
                {
                    allowShortCircuit: false,
                    allowTaggedTemplates: false,
                    allowTernary: false,
                    enforceForJSX: false,
                },
            ], // stricter than love
            "@typescript-eslint/no-unused-vars": [
                "error",
                {
                    args: "all",
                    argsIgnorePattern: "^_",
                    caughtErrors: "all",
                    ignoreRestSiblings: false,
                    vars: "all",
                },
            ], // different than love
            "@typescript-eslint/parameter-properties": ["error"],
            "@typescript-eslint/promise-function-async": ["off"],

            "@typescript-eslint/return-await": ["error", "in-try-catch"],

            "@typescript-eslint/require-await": ["error"],

            "import/consistent-type-specifier-style": ["error", "prefer-top-level"],

            "perfectionist/sort-intersection-types": ["error"],
            "perfectionist/sort-union-types": ["error"],
        },
    },

    {
        extends: [tseslint.configs.disableTypeChecked],
        files: ["*.mjs"],
        rules: {},
    },
    prettier,
);
