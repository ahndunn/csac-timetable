<script lang="ts">
  import type { FileInspection } from '../engine/excelParser';
  import { parseSelectedSheets } from '../engine/excelParser';
  import type { SongVoteData } from '../types/timetable';
  import {
    Layers,
    FileSpreadsheet,
    Check,
    X,
    Search,
    SquareCheck,
    Square,
    Users,
  } from '@lucide/svelte';
  import { tStore } from '$lib/i18n';
  import * as Dialog from '$lib/components/ui/dialog';
  import { Button } from '$lib/components/ui/button';
  import { Badge } from '$lib/components/ui/badge';
  import { Card } from '$lib/components/ui/card';
  import { Input } from '$lib/components/ui/input';

  interface Props {
    inspections: FileInspection[];
    existingCount: number;
    onClose: () => void;
    onConfirm: (selectedSongs: SongVoteData[]) => void;
  }

  let { inspections, existingCount, onClose, onConfirm }: Props = $props();

  let allValidSheetKeys = $derived.by(() => {
    const keys: string[] = [];
    for (const f of inspections) {
      for (const s of f.sheets) {
        if (s.isValid) {
          keys.push(`${f.fileId}::${s.sheetName}`);
        }
      }
    }
    return keys;
  });

  let selectedKeys = $state<Set<string>>(new Set());

  $effect(() => {
    selectedKeys = new Set(allValidSheetKeys);
  });

  let searchQuery = $state('');

  let filteredFiles = $derived.by(() => {
    const q = searchQuery.toLowerCase().trim();
    if (!q) return inspections;

    return inspections
      .map(file => {
        const matchingSheets = file.sheets.filter(s => {
          const matchTab = s.sheetName.toLowerCase().includes(q);
          const matchSong = s.songName.toLowerCase().includes(q);
          const matchMember = s.members.some(m => m.toLowerCase().includes(q));
          return matchTab || matchSong || matchMember;
        });

        return {
          ...file,
          sheets: matchingSheets
        };
      })
      .filter(f => f.sheets.length > 0);
  });

  let totalValidSheetsCount = $derived(allValidSheetKeys.length);
  let selectedCount = $derived(selectedKeys.size);

  function toggleSheet(key: string) {
    const next = new Set(selectedKeys);
    if (next.has(key)) {
      next.delete(key);
    } else {
      next.add(key);
    }
    selectedKeys = next;
  }

  function selectAll() {
    selectedKeys = new Set(allValidSheetKeys);
  }

  function deselectAll() {
    selectedKeys = new Set();
  }

  function handleConfirm() {
    const songs = parseSelectedSheets(inspections, selectedKeys, existingCount);
    onConfirm(songs);
    onClose();
  }
</script>

<Dialog.Root open={true} onOpenChange={(open) => { if (!open) onClose(); }}>
  <Dialog.Content class="max-w-2xl max-h-[90vh] overflow-y-auto">
    <Dialog.Header>
      <div class="flex items-center gap-2">
        <Layers size={20} class="text-primary" />
        <Dialog.Title class="text-base font-bold">
          {$tStore('sheet_modal.title')}
        </Dialog.Title>
      </div>
      <Dialog.Description class="text-xs text-muted-foreground">
        {$tStore('sheet_modal.intro')}
      </Dialog.Description>
    </Dialog.Header>

    <div class="flex flex-col gap-3 py-2">
      <!-- Search and Bulk Action Toolbar -->
      <div class="flex flex-wrap items-center justify-between gap-2 rounded-lg border border-border bg-muted/40 p-2.5">
        <div class="relative flex-1 min-w-[200px]">
          <Input
            type="text"
            bind:value={searchQuery}
            placeholder={$tStore('sheet_modal.search_placeholder')}
            class="h-8 text-xs pl-8"
          />
          <Search size={14} class="pointer-events-none absolute left-2.5 top-2.5 text-muted-foreground" />
        </div>

        <div class="flex items-center gap-1.5">
          <Button variant="outline" size="sm" onclick={selectAll}>
            <SquareCheck size={13} class="mr-1 text-emerald-600" />
            <span>{$tStore('sheet_modal.select_all')}</span>
          </Button>
          <Button variant="outline" size="sm" onclick={deselectAll}>
            <Square size={13} class="mr-1 text-muted-foreground" />
            <span>{$tStore('sheet_modal.deselect_all')}</span>
          </Button>
        </div>
      </div>

      <!-- File and Sheet List -->
      <div class="flex flex-col gap-3 max-h-[360px] overflow-y-auto pr-1">
        {#each filteredFiles as file (file.fileId)}
          <Card class="border border-border bg-card p-3">
            <div class="flex items-center gap-2 pb-2 border-b border-border mb-2">
              <FileSpreadsheet size={16} class="text-emerald-600" />
              <strong class="text-xs font-bold text-foreground">{file.fileName}</strong>
              <Badge variant="secondary" class="text-[10px] py-0">
                {$tStore('sheet_modal.sheets_count', { count: file.sheets.length })}
              </Badge>
            </div>

            <div class="flex flex-col gap-1.5">
              {#each file.sheets as sheet (sheet.sheetName)}
                {@const key = `${file.fileId}::${sheet.sheetName}`}
                {@const isSelected = selectedKeys.has(key)}

                <div
                  class="flex items-center justify-between rounded-lg border p-2 text-xs transition-colors cursor-pointer {sheet.isValid ? isSelected ? 'border-primary/40 bg-primary/10' : 'border-border hover:bg-muted/40' : 'border-border bg-muted/20 opacity-50 cursor-not-allowed'}"
                  onclick={() => sheet.isValid && toggleSheet(key)}
                  onkeydown={(e) => { if (e.key === 'Enter' && sheet.isValid) toggleSheet(key); }}
                  role="button"
                  tabindex={sheet.isValid ? 0 : -1}
                >
                  <div class="flex items-center gap-2">
                    {#if sheet.isValid}
                      {#if isSelected}
                        <SquareCheck size={16} class="text-primary" />
                      {:else}
                        <Square size={16} class="text-muted-foreground" />
                      {/if}
                    {:else}
                      <X size={16} class="text-destructive" />
                    {/if}

                    <div class="flex flex-col">
                      <div class="flex items-center gap-2">
                        <strong class="font-semibold text-slate-800 dark:text-slate-100">{sheet.sheetName}</strong>
                        {#if sheet.songName && sheet.songName !== sheet.sheetName}
                          <span class="text-slate-500 font-medium">({sheet.songName})</span>
                        {/if}
                      </div>

                      <div class="text-[11px] text-slate-400">
                        {#if sheet.isValid}
                          <span>{$tStore('sheet_modal.members_detected', { count: sheet.members.length, members: sheet.members.join(', ') })}</span>
                        {:else}
                          <span class="text-rose-500">Invalid structure</span>
                        {/if}
                      </div>
                    </div>
                  </div>

                  {#if sheet.isValid}
                    <Badge variant={isSelected ? 'default' : 'secondary'} class="text-[10px] py-0">
                      {isSelected ? $tStore('sheet_modal.selected') : $tStore('sheet_modal.unselected')}
                    </Badge>
                  {/if}
                </div>
              {/each}
            </div>
          </Card>
        {/each}
      </div>
    </div>

    <Dialog.Footer class="flex items-center justify-between gap-2 sm:justify-between">
      <Button variant="outline" size="sm" onclick={onClose}>
        {$tStore('sheet_modal.cancel')}
      </Button>
      <Button
        variant="default"
        size="sm"
        disabled={selectedCount === 0}
        onclick={handleConfirm}
      >
        <Check size={14} class="mr-1.5" />
        <span>{$tStore('sheet_modal.confirm_import', { selected: selectedCount, total: totalValidSheetsCount })}</span>
      </Button>
    </Dialog.Footer>
  </Dialog.Content>
</Dialog.Root>
