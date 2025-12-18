<script lang="ts">
  let {
    label,
    checked,
    onchange,
  }: {
    label: string;
    checked: boolean;
    onchange: (checked: boolean) => void;
  } = $props();
</script>

<label class="flex items-center gap-3 py-1.5 px-2 cursor-pointer rounded hover:bg-gray-100 dark:hover:bg-gray-700 transition-colors">
  <input
    type="checkbox"
    {checked}
    onchange={(e) => onchange(e.currentTarget.checked)}
    class="w-4 h-4 text-blue-600 bg-gray-100 border-gray-300
           rounded focus:ring-blue-500 dark:bg-gray-700
           dark:border-gray-600 cursor-pointer"
  />
  <span class="text-gray-700 dark:text-gray-300 font-semibold">
    {label}
  </span>
</label>
