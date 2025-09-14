import { debounce } from "@/helpers/debounce";
import useWithdrawStore from "@/store/withdraw/withdraw";
import { useCallback, useMemo } from "react";

export default function useMonthAmountWithdrawDetail(toast: any) {
    const {
        findMonthWithdrawAmountByCard,
        monthWithdrawAmount,
        loadingMonthWithdrawAmount,
        setLoadingMonthWithdrawAmount,
        setErrorMonthWithdrawAmount,
    } = useWithdrawStore();

    const monthlyAmount = useMemo(() => {
        if (!monthWithdrawAmount || !Array.isArray(monthWithdrawAmount)) {
            return Array(12).fill(0);
        }

        const balances = Array(12).fill(0);
        const year = new Date().getFullYear();

        monthWithdrawAmount.forEach((balance) => {
            const monthIndex = new Date(`${balance.month} 1, ${year}`).getMonth();
            if (balance.total_amount !== undefined) {
                balances[monthIndex] = balance.total_amount;
            }
        });

        return balances;
    }, [monthWithdrawAmount]);

    const fetchMonthlyAmount = useCallback(
        async (year: number,card_number: string) => {
            try {
                setLoadingMonthWithdrawAmount(true);
                setErrorMonthWithdrawAmount(null);

                await findMonthWithdrawAmountByCard(toast, year,card_number);
            } catch (error) {
                console.error("Failed to fetch monthly payment method:", error);
                setErrorMonthWithdrawAmount("Failed to fetch monthly payment method");
            } finally {
                setLoadingMonthWithdrawAmount(false);
            }
        },
        [
            findMonthWithdrawAmountByCard,
            setLoadingMonthWithdrawAmount,
            setErrorMonthWithdrawAmount,
        ],
    );

    const debouncedFetchMonthlyAmount = debounce(fetchMonthlyAmount, 300);

    return{
        findMonthWithdrawAmountByCard,
        monthWithdrawAmount,
        loadingMonthWithdrawAmount,
        setLoadingMonthWithdrawAmount,
        setErrorMonthWithdrawAmount,
        monthlyAmount,
        fetchMonthlyAmount,
        debouncedFetchMonthlyAmount
    }
}