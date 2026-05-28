<script>
import { page } from '$app/state';
</script>

<main class="mx-auto h-min w-max max-w-lg pretty text-center my-8">
	<h1>{page.status}: {page.error?.message}</h1>
</main>
