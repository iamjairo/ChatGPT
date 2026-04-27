import { vi } from 'vitest';

export const getCurrentWebview = vi.fn().mockReturnValue({
  label: 'titlebar',
});
