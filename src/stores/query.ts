import { derived, writable } from "svelte/store";

export const searchQuery = writable("");

// const queryWords = async (sq: string) => {
//   const words = await prisma.word.findMany({
//     where: {
//       word: {
//         startsWith: sq,
//       },
//     },
//   });

//   return words;
// };

// export const wordsQueried = derived(searchQuery, ($sq, set) => {
//   queryWords($sq).then((words) => set(words));
// });
