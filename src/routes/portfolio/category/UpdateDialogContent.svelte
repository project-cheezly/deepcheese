<script>
    import * as Dialog from "$lib/components/create-update-catalog";
    export let name, id, closeDialog;
</script>

<h3 class="py-0">수정</h3>
<form method="POST" action='?/update' class="space-y-4">
    <input name="id" type="hidden" value="{id}" />
    <Dialog.TextInput name='name' placeholder="이름" value={name} />
    <div class="text-right space-x-4 py-2">
        <button class="button-small text-gray-400" on:click|preventDefault={closeDialog}>취소</button>
        <button class="button-small">수정</button>
    </div>
</form>