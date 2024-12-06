import { getAmountList, getLists } from "./utils";

const { list1, list2 } = getLists();

const amountList2 = getAmountList(list2);

const sum = list1.reduce((acc, value) => {
  return (acc += amountList2[value] ? value * amountList2[value] : 0);
}, 0);

console.log(sum);
