# Discord facts bot
Made for a `koder support` code jam. Submitted at 4:43am UTC on 16 June 2023.
### Bot usage
Add the bot to your sever with [this invite](https://discord.com/api/oauth2/authorize?client_id=1119119614991933531&permissions=0&scope=bot) and in a channel it is in, use it's commands. Use `/help` and make sure the facts bot's command is selected (not any other bot's help command) to get started.
There is plenty of different facts to be learned!

### <a name="host-from-source"></a> Host the bot from source code
Please note: This bot was written using windows 10. Linux/mac-os/other support is **NOT** in any way guaranteed or suggested.
1. Clone this repository to a folder
2. Make sure you have installed cargo as per the rust website's directions
3. Add the required token files and fill them with the relevant api token. Ensure no spaces or newlines in the key (Trailing and preceding whitespace will be ignored). They are 
   - `discord.file` A discord bot token created in the developer portal
   - `api-ninjas-com-key.file` A api-ninjas.com token which is provided to you when you make an account there.
**Others may be added later** you can look in `.gitignore` to make an educated guess on which files to create. **If you do not provide the correct files, the bot may crash** when certain commands are run. This will hopefully be improved in future!
4. Use `cargo build` to download all the dependencies and make sure everything is in order before starting the bot. This may take a long time! Make sure you have coffee at the ready. (This step is optional: if you skip this step, all of this and a little more will be done in the next one). 
5. Run the bot with `cargo run`
6. Use control+C to terminate it
7. You can run it again any time using `cargo run` in the root directory

### Build
Follow at least steps 1 to 3 of the [Host the bot from source code](#host-from-source) instructions. Run `cargo build --release` to compile. The output file will be in `./target/release/`. On windows it will be called `facts-bot.exe`, not to be confused with facts-bot`.d` or facts_bot`.pdb`. 

Be warned, it is not portable across different types of cpus or OSs. If you are compiling for another OS, you may need libraries such as open-ssl.