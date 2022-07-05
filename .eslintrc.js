module.exports = {
	extends: 'eslint:recommended',
	env: {
		node: true,
		es2021: true
	},
	parserOptions: {
		sourceType: 'script'
	},
	rules: {
		'array-callback-return': 'error',
		'no-duplicate-imports': 'error',
		'no-use-before-define': 'error',
		camelcase: ['error', { properties: 'never' }],
		curly: 'error',
		'dot-notation': 'error',
		eqeqeq: 'error',
		'no-array-constructor': 'error',
		'no-else-return': 'error',
		'no-empty-function': 'error',
		'no-eval': 'error',
		'no-magic-numbers': ['error', { ignore: [-1, 0, 1, 2] }],
		'no-multi-assign': 'error',
		'no-return-assign': 'error',
		'no-return-await': 'error',
		'no-shadow': 'error',
		'no-throw-literal': 'error',
		'no-useless-return': 'error',
		'no-var': 'error',
		'prefer-const': 'error',
		'brace-style': 'error',
		'comma-dangle': 'error',
		'eol-last': 'error',
		indent: ['error', 'tab'],
		'linebreak-style': 'error',
		'no-multi-spaces': 'error',
		'no-multiple-empty-lines': 'error',
		'no-trailing-spaces': 'error',
		'no-control-regex': 'off',
		quotes: ['error', 'single'],
		semi: 'error'
	}
};
