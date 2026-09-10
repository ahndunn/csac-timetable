import type { Actions, PageServerLoad } from './$types';

export const load: PageServerLoad = async () => {
  return {
    meta: {
      title: 'CSAC Timetable Studio 🎵📅',
      description: 'Hệ thống xếp lịch tập tự động CSAC Music Club',
    },
  };
};

export const actions: Actions = {
  // SvelteKit SSR Form Action for handling timetable solves or server tasks
  solve: async ({ request }) => {
    const data = await request.formData();
    const maxRooms = Number(data.get('maxRooms') ?? 1);
    const allowPartialAttendance = data.get('allowPartialAttendance') === 'true';
    const spreadDays = data.get('spreadDays') !== 'false';

    return {
      success: true,
      settings: { maxRooms, allowPartialAttendance, spreadDays },
    };
  },
};
