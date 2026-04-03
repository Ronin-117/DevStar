import { useState } from 'react';
import { cn } from '../../lib/utils';
import { Checkbox } from '../shared/Checkbox';
import type { ProjectSprintSectionWithItems } from '../../lib/types';

interface CollapsibleSectionProps {
  section: ProjectSprintSectionWithItems;
  projectId: number;
  onToggleItem?: (itemId: number, projectId: number) => void;
  onAddItem?: (input: { section_id: number; title: string; description?: string }, projectId: number) => void;
  onDeleteItem?: (itemId: number, projectId: number) => void;
  onDeleteSection?: (sectionId: number, projectId: number) => void;
}

export function CollapsibleSection({
  section,
  projectId,
  onToggleItem,
  onAddItem,
  onDeleteItem,
  onDeleteSection,
}: CollapsibleSectionProps) {
  const [open, setOpen] = useState(true);
  const [addingItem, setAddingItem] = useState(false);
  const [newItemTitle, setNewItemTitle] = useState('');

  const checkedCount = section.items.filter((i) => i.checked).length;
  const totalCount = section.items.length;

  const handleAddItem = () => {
    if (!newItemTitle.trim()) return;
    onAddItem?.({ section_id: section.section.id, title: newItemTitle.trim() }, projectId);
    setNewItemTitle('');
    setAddingItem(false);
  };

  return (
    <div className="border rounded-lg overflow-hidden bg-white">
      <button
        onClick={() => setOpen(!open)}
        className="w-full flex items-center justify-between px-4 py-3 hover:bg-gray-50 transition-colors"
      >
        <div className="flex items-center gap-3">
          <svg
            className={cn('w-4 h-4 text-gray-400 transition-transform', open && 'rotate-90')}
            fill="none"
            viewBox="0 0 24 24"
            stroke="currentColor"
          >
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M9 5l7 7-7 7" />
          </svg>
          <div>
            <h3 className="font-medium text-sm text-left">{section.section.name}</h3>
            {section.section.description && (
              <p className="text-xs text-gray-500 text-left">{section.section.description}</p>
            )}
          </div>
        </div>
        <div className="flex items-center gap-3">
          <span className="text-xs text-gray-500">
            {checkedCount}/{totalCount}
          </span>
          {section.section.is_custom && onDeleteSection && (
            <button
              onClick={(e) => {
                e.stopPropagation();
                onDeleteSection(section.section.id, projectId);
              }}
              className="text-gray-400 hover:text-red-500 text-xs"
            >
              &times;
            </button>
          )}
        </div>
      </button>

      {open && (
        <div className="border-t">
          <div className="px-4 py-2 bg-gray-50">
            <div className="w-full bg-gray-200 rounded-full h-1.5 overflow-hidden">
              <div
                className="bg-indigo-600 h-1.5 rounded-full transition-all duration-300"
                style={{ width: totalCount > 0 ? `${(checkedCount / totalCount) * 100}%` : '0%' }}
              />
            </div>
          </div>

          <div className="divide-y">
            {section.items.map((item) => (
              <div
                key={item.id}
                className="flex items-start gap-3 px-4 py-2.5 hover:bg-gray-50 group"
              >
                <Checkbox
                  checked={item.checked}
                  onChange={() => onToggleItem?.(item.id, projectId)}
                />
                <div className="flex-1 min-w-0">
                  <span
                    className={cn(
                      'text-sm',
                      item.checked ? 'text-gray-400 line-through' : 'text-gray-800',
                    )}
                  >
                    {item.title}
                  </span>
                  {item.description && (
                    <p className="text-xs text-gray-500 mt-0.5">{item.description}</p>
                  )}
                </div>
                {item.is_custom && onDeleteItem && (
                  <button
                    onClick={() => onDeleteItem(item.id, projectId)}
                    className="opacity-0 group-hover:opacity-100 text-gray-400 hover:text-red-500 text-xs shrink-0"
                  >
                    &times;
                  </button>
                )}
              </div>
            ))}
          </div>

          {addingItem ? (
            <div className="px-4 py-2 border-t flex gap-2">
              <input
                value={newItemTitle}
                onChange={(e) => setNewItemTitle(e.target.value)}
                onKeyDown={(e) => e.key === 'Enter' && handleAddItem()}
                placeholder="Item title..."
                className="flex-1 text-sm border rounded px-2 py-1 focus:outline-none focus:ring-1 focus:ring-indigo-500"
                autoFocus
              />
              <button
                onClick={handleAddItem}
                className="text-xs px-2 py-1 bg-indigo-600 text-white rounded hover:bg-indigo-700"
              >
                Add
              </button>
              <button
                onClick={() => {
                  setAddingItem(false);
                  setNewItemTitle('');
                }}
                className="text-xs px-2 py-1 border rounded hover:bg-gray-50"
              >
                Cancel
              </button>
            </div>
          ) : (
            <button
              onClick={() => setAddingItem(true)}
              className="w-full text-left px-4 py-2 text-sm text-indigo-600 hover:bg-gray-50 border-t"
            >
              + Add item
            </button>
          )}
        </div>
      )}
    </div>
  );
}
