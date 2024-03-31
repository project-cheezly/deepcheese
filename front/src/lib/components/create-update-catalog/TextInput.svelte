<script>
    export let value = '', name, placeholder;
</script>

<label class="flex w-full flex-col gap-1">
    <span class="font-semibold">{placeholder}</span>
    <input class="border px-3 py-2 font-semibold" name={name} placeholder={placeholder} value={value} />
</label>
