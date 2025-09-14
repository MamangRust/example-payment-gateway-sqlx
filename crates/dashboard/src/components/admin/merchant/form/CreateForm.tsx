import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  CreateMerchantFormValues,
  createMerchantRequestSchema,
} from "@/schemas";
import useUserStore from "@/store/user/user";
import { MerchantCreateFormProps } from "@/types/form";
import { zodResolver } from "@hookform/resolvers/zod";
import { forwardRef, useCallback, useEffect, useState } from "react";
import { useForm, Controller } from "react-hook-form";
import AsyncSelect from "react-select/async";
import { debounce } from "@/helpers/debounce";
import { useToast } from "@/hooks/use-toast";
import { FindAllUser } from "@/types/domain/request";

interface UserOption {
  value: string;
  label: string;
}

const CreateMerchantForm = forwardRef<HTMLFormElement, MerchantCreateFormProps>(
  ({ onSubmit }, ref) => {
    const {
      register,
      control,
      handleSubmit,
      formState: { errors },
    } = useForm<CreateMerchantFormValues>({
      resolver: zodResolver(createMerchantRequestSchema),
      defaultValues: {
        name: "",
        user_id: 0,
      },
    });

    const { toast } = useToast();

    const { users, loadingGetUsers, setLoadingGetUsers, findAllUsers } =
      useUserStore();

    const [userOptions, setUserOptions] = useState<UserOption[]>([]);

    useEffect(() => {
      const fetchInitialUsers = async () => {
        setLoadingGetUsers(true);
        try {
          const searchReq: FindAllUser = {
            search: "",
            page: 1,
            page_size: 10,
            toast: toast,
          };
          await findAllUsers(searchReq);

          if (users) {
            const options = users.map((user) => ({
              value: user.id.toString(),
              label: `${user.firstname} ${user.lastname}` || user.email,
            }));
            setUserOptions(options);
          }
        } catch (error) {
          console.error("Failed to fetch initial users:", error);
        } finally {
          setLoadingGetUsers(false);
        }
      };

      fetchInitialUsers();
    }, []);

    const loadOptions = useCallback(
      debounce(
        async (
          inputValue: string,
          callback: (options: UserOption[]) => void,
        ) => {
          setLoadingGetUsers(true);
          try {
            const searchReq: FindAllUser = {
              search: inputValue,
              page: 1,
              page_size: 10,
              toast: toast,
            };
            await findAllUsers(searchReq);

            if (users) {
              const filteredOptions = users
                .filter(
                  (user) =>
                    `${user.firstname} ${user.lastname}`
                      .toLowerCase()
                      .includes(inputValue.toLowerCase()) ||
                    user.email.toLowerCase().includes(inputValue.toLowerCase()),
                )
                .map((user) => ({
                  value: user.id.toString(),
                  label: `${user.firstname} ${user.lastname}` || user.email,
                }));

              callback(filteredOptions);
            }
          } catch (error) {
            console.error("Failed to fetch users:", error);
          } finally {
            setLoadingGetUsers(false);
          }
        },
        500,
      ),
      [users, findAllUsers, toast],
    );

    return (
      <form ref={ref} onSubmit={handleSubmit(onSubmit)} className="space-y-4">
        <div>
          <Label htmlFor="name" className="text-sm font-medium">
            Name
          </Label>
          <Input
            id="name"
            type="text"
            placeholder="Enter merchant name"
            className="mt-1"
            {...register("name")}
          />
          {errors.name && (
            <p className="text-red-500 text-sm mt-1">{errors.name.message}</p>
          )}
        </div>
        <div>
          <Label htmlFor="userId" className="text-sm font-medium">
            User ID
          </Label>
          <Controller
            name="user_id"
            control={control}
            render={({ field }) => (
              <AsyncSelect
                {...field}
                loadOptions={loadOptions}
                defaultOptions={userOptions}
                isLoading={loadingGetUsers}
                placeholder="Select a user"
                className="mt-1"
                styles={{
                  control: (base) => ({
                    ...base,
                    borderColor: errors.user_id ? "#ef4444" : base.borderColor,
                    borderRadius: "0.375rem",
                    padding: "0.25rem",
                    boxShadow: "none",
                    "&:hover": {
                      borderColor: "#3b82f6",
                    },
                  }),
                  option: (base, { isFocused, isSelected }) => ({
                    ...base,
                    backgroundColor: isSelected
                      ? "#3b82f6"
                      : isFocused
                        ? "#bfdbfe"
                        : "white",
                    color: isSelected ? "white" : "black",
                    "&:hover": {
                      backgroundColor: "#bfdbfe",
                    },
                  }),
                  menu: (base) => ({
                    ...base,
                    borderRadius: "0.375rem",
                    boxShadow: "0 4px 6px -1px rgb(0 0 0 / 0.1)",
                  }),
                }}
                value={userOptions.find(
                  (option) => Number(option.value) === field.value,
                )}
                onChange={(selectedOption) => {
                  field.onChange(
                    selectedOption ? Number(selectedOption.value) : null,
                  );
                }}
              />
            )}
          />
          {errors.user_id && (
            <p className="text-red-500 text-sm mt-1">
              {errors.user_id.message}
            </p>
          )}
        </div>
      </form>
    );
  },
);

CreateMerchantForm.displayName = "CreateMerchantForm";

export default CreateMerchantForm;
