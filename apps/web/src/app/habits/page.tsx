import { getHabits } from '@/services/habits';

export default async function HabitsPage() {
  const habits = await getHabits();

  return (
    <main>
      <h1>Habits Page</h1>
      <ul>
        {habits.map((habit) => (
          <li key={habit.name}>{habit.name}</li>
        ))}
      </ul>
    </main>
  );
}
