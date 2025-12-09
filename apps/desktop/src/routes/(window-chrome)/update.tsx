// 离线版：已禁用自动更新功能
import { Button } from "@cap/ui-solid";
import { useNavigate } from "@solidjs/router";

export default function () {
	const navigate = useNavigate();

	return (
		<div class="flex flex-col justify-center flex-1 items-center gap-[3rem] p-[1rem] text-[0.875rem] font-[400] h-full">
			<div class="flex flex-col gap-4 items-center">
				<p class="text-[--text-tertiary]">
					离线版暂不支持自动更新
				</p>
				<Button onClick={() => navigate("/")}>返回</Button>
			</div>
		</div>
	);
}
