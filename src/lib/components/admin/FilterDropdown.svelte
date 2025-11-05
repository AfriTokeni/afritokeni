<script lang="ts">
  import { ChevronDown } from "@lucide/svelte";
  import { Button, Dropdown, DropdownItem } from "flowbite-svelte";

  interface Option {
    value: string;
    label: string;
  }

  interface Props {
    value: string;
    options: Option[];
    onChange?: (value: string) => void;
  }

  let { value = $bindable(""), options, onChange }: Props = $props();
  
  let selectedLabel = $derived(
    options.find(opt => opt.value === value)?.label || options[0]?.label || "Select"
  );
  
  function selectOption(optionValue: string) {
    value = optionValue;
    onChange?.(value);
  }
</script>

<div class="relative">
  <Button size="sm" color="light" class="gap-2">
    {selectedLabel}
    <ChevronDown class="h-4 w-4" />
  </Button>
  <Dropdown placement="bottom-start" class="z-50 w-44 !shadow-md">
    {#each options as option}
      <DropdownItem onclick={() => selectOption(option.value)}>
        {option.label}
      </DropdownItem>
    {/each}
  </Dropdown>
</div>
