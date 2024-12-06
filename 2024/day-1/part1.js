import { getLists } from "./utils";

const { list1, list2 } = getLists();
const sortedList1 = list1.toSorted();
const sortedList2 = list2.toSorted();

const sum = sortedList1.reduce((acc, _, index) => {
  return (acc += Math.abs(sortedList2[index] - sortedList1[index]));
}, 0);

console.log(sum);
