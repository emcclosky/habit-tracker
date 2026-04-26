import { apiFetch } from '@/lib/client';
import type { Habit } from '@habit-tracker/shared';

export const getHabits = () => apiFetch<Habit[]>('/habits');

export const createHabit = (name: string) =>
  apiFetch<Habit>('/habits', {
    method: 'POST',
    body: JSON.stringify({ name }),
  });

export const completeHabit = (name: string, date: string) =>
  apiFetch<Habit>(`/habits/${encodeURIComponent(name)}/completions/${date}`, {
    method: 'POST',
  });

export const deleteCompletion = (name: string, date: string) =>
  apiFetch<Habit>(`/habits/${encodeURIComponent(name)}/completions/${date}`, {
    method: 'DELETE',
  });
