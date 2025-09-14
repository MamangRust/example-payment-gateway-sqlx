import { debounce } from "@/helpers/debounce";
import useCardStore from "@/store/card/card";
import { useCallback, useMemo } from "react";


export default function useMonthWithdrawCard(toast: any) {
    const {
        findMonthWithdrawAmount,
        monthWithdrawAmount,
        loadingMonthWithdrawAmount,
        setLoadingMonthWithdrawAmount,
        setErrorMonthWithdrawAmount,
    } = useCardStore();

    const monthlyWithdraws = useMemo(() => {
        if (!monthWithdrawAmount || !Array.isArray(monthWithdrawAmount)) {
            return Array(12).fill(0);
        }

        const withdraws = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthWithdrawAmount.forEach((withdraw) => {
            const monthIndex = new Date(`${withdraw.month} 1, ${year}`).getMonth();
            if (withdraw.total_amount !== undefined) {
                withdraws[monthIndex] = withdraw.total_amount;
            }
        });

        return withdraws;
    }, [monthWithdrawAmount]);

    const fetchMonthlyWithdraws = useCallback(
        async (year: number) => {
            try {
                setLoadingMonthWithdrawAmount(true);
                setErrorMonthWithdrawAmount(null);

                await findMonthWithdrawAmount(toast, year);
            } catch (error) {
                console.error("Failed to fetch monthly withdraws:", error);
                setErrorMonthWithdrawAmount("Failed to fetch monthly withdraws");
            } finally {
                setLoadingMonthWithdrawAmount(false);
            }
        },
        [
            findMonthWithdrawAmount,
            setLoadingMonthWithdrawAmount,
            setErrorMonthWithdrawAmount,
        ],
    );

    const debouncedFetchMonthlyWithdraws = debounce(fetchMonthlyWithdraws, 1000);

    return {
        findMonthWithdrawAmount,
        monthWithdrawAmount,
        loadingMonthWithdrawAmount,
        setLoadingMonthWithdrawAmount,
        setErrorMonthWithdrawAmount,
        monthlyWithdraws,
        fetchMonthlyWithdraws,
        debouncedFetchMonthlyWithdraws
    }
}