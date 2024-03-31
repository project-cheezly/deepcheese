<script lang="ts">
  import { DatePicker } from "bits-ui";

  export let placeholder = "일시";
  let value = "";
</script>

<DatePicker.Root weekdayFormat="short" fixedWeeks={true} bind:value>
  <div class="flex w-full flex-col gap-1.5">
    <span class="font-semibold">{placeholder}</span>
    <DatePicker.Input
      let:segments
      class="flex w-full select-none items-center border border-border-input bg-background px-2 py-1 text-foreground focus-within:border-border-input-hover focus-within:shadow-date-field-focus hover:border-border-input-hover"
    >
      {#each segments.sort((key) => ['day', 'month', 'literal'].indexOf(key.part)) as { part, value }}
        <div class="inline-block select-none">
          {#if part !== "literal"}
            <DatePicker.Segment
              {part}
              class="px-1 hover:bg-muted focus:bg-muted focus:text-foreground focus-visible:!ring-0 focus-visible:!ring-offset-0 aria-[valuetext=Empty]:text-muted-foreground"
            >
              {value}{part === "year" ? "년" : part === "month" ? "월" : "일"}
            </DatePicker.Segment>
          {/if}
        </div>
      {/each}
      <DatePicker.Trigger
        class="ml-auto grow-0 inline-flex size-8 items-center justify-center text-foreground/60 transition-all hover:bg-muted active:bg-dark-10"
      >
        일시
      </DatePicker.Trigger>
    </DatePicker.Input>
    <DatePicker.Content
      class="z-50"
    >
      <DatePicker.Calendar
        class="border border-dark-10 bg-background p-[22px] shadow-popover"
        let:months
        let:weekdays
      >
        <DatePicker.Header class="flex items-center justify-between">
          <DatePicker.PrevButton
            class="text-sm inline-flex size-10 items-center justify-center bg-background-alt transition-all hover:bg-muted active:scale-98"
          >
            이전달
          </DatePicker.PrevButton>
          <DatePicker.Heading class="text-[15px] font-medium" />
          <DatePicker.NextButton
            class="text-sm inline-flex size-10 items-center justify-center bg-background-alt transition-all hover:bg-muted active:scale-98"
          >
            다음달
          </DatePicker.NextButton>
        </DatePicker.Header>
        <div
          class="flex flex-col space-y-4 pt-4 sm:flex-row sm:space-x-4 sm:space-y-0"
        >
          {#each months as month}
            <DatePicker.Grid
              class="w-full border-collapse select-none space-y-1"
            >
              <DatePicker.GridHead>
                <DatePicker.GridRow class="mb-1 flex w-full justify-between">
                  {#each weekdays as day}
                    <DatePicker.HeadCell
                      class="w-10 rounded-md text-xs !font-normal text-muted-foreground"
                    >
                      <div>{day.slice(0, 2)}</div>
                    </DatePicker.HeadCell>
                  {/each}
                </DatePicker.GridRow>
              </DatePicker.GridHead>
              <DatePicker.GridBody>
                {#each month.weeks as weekDates}
                  <DatePicker.GridRow class="flex w-full">
                    {#each weekDates as date}
                      <DatePicker.Cell
                        {date}
                        class="relative size-10 !p-0 text-center text-sm"
                      >
                        <DatePicker.Day
                          {date}
                          month={month.value}
                          class="group relative inline-flex size-10 items-center justify-center whitespace-nowrap rounded-9px border border-transparent bg-transparent p-0 text-sm font-normal text-foreground transition-all hover:border-foreground data-[disabled]:pointer-events-none data-[outside-month]:pointer-events-none data-[selected]:bg-foreground data-[selected]:font-medium data-[disabled]:text-foreground/30 data-[selected]:text-background data-[unavailable]:text-muted-foreground data-[unavailable]:line-through"
                        >
                          <div
                            class="absolute top-[5px] hidden size-1 rounded-full bg-foreground transition-all group-data-[today]:block group-data-[selected]:bg-background"
                          />
                          {date.day}
                        </DatePicker.Day>
                      </DatePicker.Cell>
                    {/each}
                  </DatePicker.GridRow>
                {/each}
              </DatePicker.GridBody>
            </DatePicker.Grid>
          {/each}
        </div>
      </DatePicker.Calendar>
    </DatePicker.Content>
  </div>
  <input type="hidden" name="record_date" value={value} />
</DatePicker.Root>
