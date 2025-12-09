import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";

export default function useMonthTopupCard(card_number: string,toast: any) {
    const {
        findMonthlyTopupAmountByCard,
        monthTopupAmount,
        loadingMonthTopupAmount,
        setLoadingMonthTopupAmount,
        setErrorMonthTopupAmount,
    } = useCardStore();

    const monthlyTopups = useMemo(() => {
        if (!monthTopupAmount || !Array.isArray(monthTopupAmount)) {
            return Array(12).fill(0);
        }

        const topups = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthTopupAmount.forEach((topup) => {
            const monthIndex = new Date(`${topup.month} 1, ${year}`).getMonth();
            if (topup.total_amount !== undefined) {
                topups[monthIndex] = topup.total_amount;
            }
        });

        return topups;
    }, [monthTopupAmount]);

    const fetchMonthlyTopups = useCallback(
        async (year: number) => {
          try {
            setLoadingMonthTopupAmount(true);
            setErrorMonthTopupAmount(null);
    
            await findMonthlyTopupAmountByCard(toast, year, card_number as string);
          } catch (error) {
            console.error("Failed to fetch monthly topups:", error);
            setErrorMonthTopupAmount("Failed to fetch monthly topups");
          } finally {
            setLoadingMonthTopupAmount(false);
          }
        },
        [
          findMonthlyTopupAmountByCard,
          setLoadingMonthTopupAmount,
          setErrorMonthTopupAmount,
        ],
      );

    const debouncedFetchMonthlyTopups = debounce(fetchMonthlyTopups, 1000);

    return {
        findMonthlyTopupAmountByCard,
        monthTopupAmount,
        loadingMonthTopupAmount,
        setLoadingMonthTopupAmount,
        setErrorMonthTopupAmount,
        monthlyTopups,
        fetchMonthlyTopups,
        debouncedFetchMonthlyTopups
    }
}