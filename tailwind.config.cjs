/** @type {import('tailwindcss').Config} */
module.exports = {
	content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
	theme: {
		extend: {
			backgroundImage: {
				greenish: "linear-gradient(90deg, #f5cb5c 0%, #64da99 50%)",
				blueish: "linear-gradient(90deg, #b42bf8 0%, #6489da 50%)",
				reddish: "linear-gradient(90deg, #fd2323 0%, #da754f 50%)",
			},
			colors: {
				background: "#1a1a1a",
				foreground: "#ffffff",
			},
			spacing: {
				maxwidth: "100rem"
			}

		},
	},
	plugins: [],
}
