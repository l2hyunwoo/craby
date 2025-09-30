import { program } from '@commander-js/extra-typings';
import { version } from '../package.json';
import { command as buildCommand } from './commands/build';
import { command as cleanCommand } from './commands/clean';
import { command as codegenCommand, runCodegen } from './commands/codegen';
import { command as doctorCommand } from './commands/doctor';
import { command as initCommand } from './commands/init';
import { command as showCommand } from './commands/show';

export function run(baseCommand: string) {
  const cli = program.name(baseCommand).version(version);

  cli.addCommand(codegenCommand);
  cli.addCommand(initCommand);
  cli.addCommand(buildCommand);
  cli.addCommand(showCommand);
  cli.addCommand(doctorCommand);
  cli.addCommand(cleanCommand);

  cli.argument('[args...]', 'optional arguments').action((args) => {
    if (args && args.length > 0) {
      console.error(`error: unknown command '${args[0]}'`);
      cli.help();
    } else {
      runCodegen();
    }
  });

  cli.parse();
}
