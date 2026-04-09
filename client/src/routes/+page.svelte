<script>
import Marquee from "$lib/Marquee.svelte";
import Hero from "$lib/Hero.svelte";
</script>

<Hero />
<main class="mx-auto overflow-hidden h-min flex flex-col min-h-screen">

	<div id="maindiv" class="flex flex-col w-full grow" >
		<div class="flex flex-col mx-auto my-8 px-2 w-sm pretty sans" >
			<Marquee
				static="I'm a(n)"
				text="software developer, systems engineer, tinkerer, back-end specialist, nerd, tutor, supporter, rustacean, arch user btw, minimalist, maximalist, "
			/>
			<div class="mx-auto text-center">
				<p class="my-0! smooth-theme-change">
					and I'm happy to share my little corner of the internet with you.
					<!-- or -->
					<!-- <a href="/blog" class="pretty-button smooth-theme-change box-decoration-clone p-1! -m-1!" draggable="false">take a peek at what I've been up to recently.</a> -->
				</p>
				<a href="/projects" class="pretty-button smooth-theme-change box-decoration-clone p-1! -m-1!" draggable="false">Come check out what I've made!</a>
			</div>
		</div>
		<div class="pretty w-full text-center">
			<h3 class="smooth-theme-change">Contact Me</h3>
		</div>
		<div class="flex flex-row flex-wrap justify-center mx-auto w-sm md:w-2xl pretty sans">
			<a
				href="/assets/resume.pdf"
				class="pretty-button smooth-theme-change no-underline!"
				draggable="false"
			>
				Resume
			</a>
			<a
				href="https://github.com/apetbrz"
				class="pretty-button smooth-theme-change no-underline!"
				draggable="false"
			>
				GitHub
			</a>
			<a
				href="https://www.linkedin.com/in/arthur-t-petroff/"
				class="pretty-button smooth-theme-change no-underline!"
				draggable="false"
			>
				LinkedIn
			</a>
			<a
				href="mailto:apetbrz@gmail.com"
				class="pretty-button smooth-theme-change no-underline!"
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
