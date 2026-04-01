<script>
import Marquee from "$lib/Marquee.svelte";

//TODO: HOW TO MAKE tabindex DYNAMIC WITH SIZE? for maindiv ordering (content -> links on <md, links -> content on >=md)
</script>

<main class="mx-auto overflow-hidden h-min flex flex-col min-h-screen">

	<div
		id="maindiv"
		class="flex flex-col w-full grow"
	>
		<div
			class="flex flex-col gap-4 mx-auto my-8 px-2 w-sm pretty sans"
		>
			<Marquee
				static="I'm a(n)"
				text="software developer, systems engineer, tinkerer, back-end specialist, nerd, tutor, supporter, rustacean, arch user btw, minimalist, maximalist, "
			/>
			<div class="mx-auto">
				<a href="/projects" class="pretty-button-nofocus" draggable="false">
					Check out some things I'm working on
				</a>
			</div>
		</div>
		<div class="pretty w-full text-center"><h3>Contact Me</h3></div>
		<div class="flex flex-row flex-wrap justify-center mx-auto w-sm md:w-2xl pretty sans">
			<a
				href="/assets/resume.pdf"
				class="pretty-button-nofocus no-underline!"
				draggable="false"
			>
				Resume
			</a>
			<a
				href="https://github.com/apetbrz"
				class="pretty-button-nofocus no-underline!"
				draggable="false"
			>
				GitHub
			</a>
			<a
				href="https://www.linkedin.com/in/arthur-t-petroff/"
				class="pretty-button-nofocus no-underline!"
				draggable="false"
			>
				LinkedIn
			</a>
			<a
				href="mailto:apetbrz@gmail.com"
				class="pretty-button-nofocus no-underline!"
				draggable="false"
			>
				Email
			</a>
		</div>
	</div>
</main>

<style lang="postcss">
@reference "tailwindcss";
</style>
