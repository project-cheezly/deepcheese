<script>
    import * as Dialog from '$lib/components/create-update-catalog';

    export let closeDialog;
    export let accounts, categories;

    $: accountList = Object.entries(accounts).reduce((acc, [key, value]) => {
        acc.push({id: key, name: value});
        return acc;
    }, []);

    $: categoryList = Object.entries(categories).reduce((acc, [key, value]) => {
        acc.push({id: key, name: value});
        return acc;
    }, []);

    const currencyList = [
        { id: 1, name: 'KRW' },
        { id: 2, name: 'USD' },
    ];

    const transactionType = [
        { id: 'INFLOW', name: '입금' },
        { id: 'OUTFLOW', name: '출금' }
    ];
</script>

<h3 class="py-0">추가</h3>
<form method="POST" action="?/create" class="pt-4">
    <div class="space-y-4">
        <Dialog.DateInput />
        <Dialog.TouchSelectInput items={accountList} name="account_id" title="계좌" />
        <Dialog.TouchSelectInput items={categoryList} name="category_id" title="카테고리" />
        <Dialog.TouchSelectInput items={transactionType} name="transaction_type" title="거래유형" />
        <Dialog.TouchSelectInput items={currencyList} name="currency_id" title="통화" />
        <Dialog.TextInput name="amount" placeholder="금액" />
        <div class="text-right space-x-4 py-2">
            <button class="button-small text-gray-400" on:click|preventDefault={closeDialog}>취소</button>
            <button class="button-small">추가</button>
        </div>
    </div>
</form>