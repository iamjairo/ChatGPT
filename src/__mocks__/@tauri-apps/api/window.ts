import { vi } from 'vitest';

export const getCurrentWindow = vi.fn().mockReturnValue({
  theme: vi.fn().mockResolvedValue('light'),
  onThemeChanged: vi.fn().mockResolvedValue(vi.fn()),
  isFullscreen: vi.fn().mockResolvedValue(false),
  listen: vi.fn().mockResolvedValue(vi.fn()),
});

export const getCurrentWebview = vi.fn().mockReturnValue({
  label: 'titlebar',
});
