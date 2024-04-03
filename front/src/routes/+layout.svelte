<script>
    import '../app.css';

	export let data;
</script>

<div class="sticky top-0 border-b bg-white main-container main-content grid grid-cols-2">
	<h3 class="font-bold col-span-1"><a href="/">Cheezly</a></h3>
	<p class="text-sm text-right col-span-1 my-auto">
		업데이트 | {new Intl.DateTimeFormat('ko-KR', {
			timeStyle: 'short',
			dateStyle: 'short',
		}).format(data.updatedAt)}
	</p>
</div>

<slot />
